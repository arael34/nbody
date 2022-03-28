/*
TODO:

add fps
*/

mod quadtree;

use std::f32::consts::PI;

use rand::Rng;
use quadtree::{Point, Bound, QuadTree, Position};
use ggez::{
            event::EventHandler, 
            GameResult, 
            Context, 
            graphics, 
            graphics::Color, 
            ContextBuilder, 
            event,
            conf::{WindowMode, WindowSetup},
            mint::Point2,
        };

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;
const FPS: u8 = 60;
const G: f32 = 5.;

#[derive(Copy, Clone)]
struct OrbitalBody {
    pos: (f32, f32),
    mass: f32,
    vel: (f32, f32),
    acc: (f32, f32),
}

impl OrbitalBody {
    fn new(pos: (f32, f32), mass: f32, vel: (f32, f32)) -> Self {
        OrbitalBody {pos, mass, vel, acc: (0., 0.) }
    }
    fn check(&mut self, others: Vec<&OrbitalBody>) -> () {
        let mut force_x: f32 = 0.;
        let mut force_y: f32 = 0.;
        for other in others {
            let (distance, angle) = find_distance_angle(self.position(), other.position());
            let gravity = self.mass * other.mass / distance.powi(2);
            force_x += gravity * angle.cos();
            force_y += gravity * angle.sin();
        }
        self.acc.0 += force_x / self.mass * 10f32.powi(-11);
        self.acc.1 += force_y / self.mass * 10f32.powi(-11);
    }
}

impl Position for OrbitalBody {
    fn position(&self) -> Point {
        (self.pos.0.into(), self.pos.1.into())
    }
}

struct Simulation {
    items: Vec<OrbitalBody>,
    qt: QuadTree<OrbitalBody>,
}

impl Simulation {
    fn new(_ctx: &mut Context, items: Vec<OrbitalBody>, qt: QuadTree<OrbitalBody>) -> Self {
        Simulation{ items, qt }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for i in &mut self.items {
            //i.check(self.qt.query(&Bound::new((i.pos.0 as f64, i.pos.1 as f64), 5., 5.)));
            i.vel.0 += i.acc.0;
            i.vel.1 += i.acc.1;
            let mut new_x = i.pos.0 + i.vel.0;
            let mut new_y = i.pos.1 + i.vel.1;
            if new_x > WIDTH || new_x < 0. {
                new_x = max(0., min(WIDTH, new_x));
                i.vel.0 = -i.vel.0;  
            }
            if new_y > HEIGHT || new_y < 0. {
                new_y = max(0., min(HEIGHT, new_y));
                i.vel.1 = -i.vel.1;
            }
            i.pos = (new_x, new_y);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        for i in &self.items {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2{ x: i.pos.0, y: i.pos.1 },
                i.mass,
                0.05,
                [0., 0.5, 1., 1.].into(),
            )?;
            graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;
        }
        graphics::present(ctx)
        }
}
       
#[allow(unused_variables, unused_mut)]
fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("Simulation", "")
        .window_setup(WindowSetup::default().title("Simulation"))
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT))
        .build()
        .expect("Failed to create context.");

    let mut qt = QuadTree::<OrbitalBody>::new(
                                Bound::new(
                                ((WIDTH / 2.).into(), (HEIGHT / 2.).into()), 
                                (WIDTH / 2.).into(), 
                                (HEIGHT / 2.).into()
                                    ));
    let mut rng = rand::thread_rng();
    let mut os = vec![];
    let mut ps = vec![];
    for i in 0..=10 {
        let o = OrbitalBody::new((rng.gen::<f32>() * WIDTH, rng.gen::<f32>() * HEIGHT), rng.gen::<f32>() * 5. + 2., (rng.gen::<f32>(), rng.gen::<f32>()));
        os.push(o.clone());
        ps.push(o);
    }
    let mut simulation = Simulation::new(&mut ctx, os, qt);
    simulation.qt.insert_all(ps);
    event::run(ctx, event_loop, simulation);
}

// Helper functions
fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn find_distance_angle(a: (f64, f64), b: (f64, f64)) -> (f32, f32) {
    (((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt() as f32,
    (b.1 - a.1).atan2(b.0 - a.0) as f32)
}

#[cfg(test)]
mod test {
    use super::{find_distance_angle, max, min};
    #[test]
    fn test_distance() {
        assert_eq!(find_distance_angle((0., 0.), (2., 0.)), (2., 0.));
    }
    #[test]
    fn test_max() {
        assert_eq!(max(5.0, 3.0), 5.0);
    }
    #[test]
    fn test_min() {
        assert_eq!(min(40.0, 10.0), 10.0);
    }
}
