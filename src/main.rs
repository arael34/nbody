/*
TODO:

add fps
*/

mod quadtree;

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

#[derive(Copy, Clone)]
struct OrbitalBody {
    pos: (f32, f32),
    mass: f32,
    vel: f32,
    acc: f32,
    ang: f32,
}

impl OrbitalBody {
    fn new(pos: (f32, f32), mass: f32, ang: f32) -> Self {
        OrbitalBody {pos, mass, vel: 0., acc: 0., ang }
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
            let mut new_x = i.pos.0 + i.vel * i.ang.cos();
            let mut new_y = i.pos.1 + i.vel * i.ang.sin();
            if new_x > WIDTH || new_x < 0. {
                new_x = max(0., min(WIDTH, new_x));
            }
            if new_y > HEIGHT || new_y < 0. {
                new_y = max(0., min(HEIGHT, new_y));
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

    let o = OrbitalBody::new((400., 400.), 5., 0.);
    let r = OrbitalBody::new((200., 100.), 2., 0.);
    let mut simulation = Simulation::new(&mut ctx, vec![o.clone(), r.clone()], qt);
    simulation.qt.insert_all(vec![o, r]);
    event::run(ctx, event_loop, simulation);
}

fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
