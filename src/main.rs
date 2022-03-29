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
            graphics::{Color, Rect}, 
            ContextBuilder, 
            event,
            conf::{WindowMode, WindowSetup},
            mint::Point2,
        };

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;
const FPS: u8 = 60;
const G: f32 = 0.00000000667;

#[derive(Copy, Clone)]
struct OrbitalBody {
    pos: (f32, f32),
    mass: f32,
    vel: (f32, f32),
    acc: (f32, f32),
}

impl OrbitalBody {
    fn new(pos: (f32, f32), vel: (f32, f32)) -> Self {
        OrbitalBody {pos, mass: 5.97 * 1024., vel, acc: (0., 0.) }
    }
    fn check(&mut self, others: Vec<&OrbitalBody>) -> () {
        let mut force_x: f32 = 0.;
        let mut force_y: f32 = 0.;
        for other in others {
            let (distance, mut angle) = find_distance_angle(self.position(), other.position());
            if distance != 0. {
                let gravity = G * self.mass * other.mass / distance.powi(2);
                force_x += gravity * angle.cos();
                force_y += gravity * angle.sin();
            }
        }
        self.acc.0 += force_x;
        self.acc.1 += force_y;
    }
}

impl Position for OrbitalBody {
    fn position(&self) -> Point {
        (self.pos.0 as f64, self.pos.1 as f64)
    }
}

struct Simulation {
    qt: QuadTree<OrbitalBody>,
}

impl Simulation {
    fn new(_ctx: &mut Context, qt: QuadTree<OrbitalBody>) -> Self {
        Simulation{ qt }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut new_bodies = Vec::<OrbitalBody>::new();
        for i in self.qt.query_all() {
            let mut a = *i;
            //a.check(self.qt.query(&Bound::new((a.pos.0 as f64, a.pos.1 as f64), 20., 20.)));
            a.check(self.qt.query_all());
            a.vel.0 += a.acc.0;
            a.vel.1 += a.acc.1;
            //if a.vel.0 > 3. { a.vel.0 = 3. }
            //if a.vel.1 > 3. { a.vel.1 = 3. }
            a.pos.0 += a.vel.0;
            a.pos.1 += a.vel.1;
            if a.pos.0 > WIDTH - 1. || a.pos.0 < 1. {
                a.pos.0 = max(1., min(WIDTH - 1., a.pos.0));
                a.vel.0 = -a.vel.0;  
            }
            if a.pos.1 > HEIGHT - 1. || a.pos.1 < 1. {
                a.pos.1 = max(1., min(HEIGHT - 1., a.pos.1));
                a.vel.1 = -a.vel.1;
            }
            new_bodies.push(a);
        }
        self.qt.clear();
        self.qt.insert_all(new_bodies);
        // print!("{} ", self.qt.items[0].pos.0);
        //print!("{} ", self.qt.items[0].pos.1.trunc() as i32);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        for i in self.qt.query_all() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2{ x: i.pos.0, y: i.pos.1 },
                4.,
                0.05,
                [0., 0.5, 1., 1.].into(),
            )?;
            graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;
        }
        for t in self.qt.get_trees() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::stroke(2.), 
                Rect::new((t.bounds.pos.0 - t.bounds.half_x) as f32, (t.bounds.pos.0 - t.bounds.half_y) as f32, t.bounds.half_x as f32 * 2., t.bounds.half_y as f32* 2.), 
                [1., 1., 1., 1.,].into(),
            )?;
            graphics::draw(ctx, &rectangle, (Point2 { x: 0.0, y: 0.0 },))?;
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
    let mut ps = vec![];
    for i in 0..4 {
        let o = OrbitalBody::new((rng.gen::<f32>() * WIDTH, rng.gen::<f32>() * HEIGHT), (rng.gen::<f32>() * 2. - 1., rng.gen::<f32>() * 2. - 1.));
        ps.push(o);
    }
    let mut simulation = Simulation::new(&mut ctx, qt);
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
    use std::f32::consts::PI;

    use super::{find_distance_angle, max, min};
    #[test]
    fn test_distance() {
        assert_eq!(find_distance_angle((0., 0.), (-3., -3.)).1, find_distance_angle((-3., -3.), (0., 0.)).1 + PI);
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
