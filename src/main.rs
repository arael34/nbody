/*
TODO:


*/

mod quadtree;

use quadtree::{Point, Bound, QuadTree, Position};
use ggez::{
            event::EventHandler, 
            GameResult, 
            Context, 
            graphics, 
            graphics::{Color, Mesh}, 
            ContextBuilder, 
            event,
            conf::{WindowMode, WindowSetup},
            mint::Point2,
        };

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;
const FPS: u8 = 60;

#[allow(dead_code, unused_variables)]
struct OrbitalBody {
    pos: (f32, f32),
    mass: f32,
}

impl OrbitalBody {
    fn new(pos: (f32, f32), mass: f32) -> Self {
        OrbitalBody {pos, mass}
    }
}

impl Position for OrbitalBody {
    fn position(&self) -> Point {
        (self.pos.0.into(), self.pos.1.into())
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
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        for i in self.qt.query_all() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2{ x: i.pos.0, y: i.pos.1 },
                i.mass,
                5.,
                [0.3, 0.3, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;
        }
        graphics::present(ctx)
        }
}
       
#[allow(unused_variables, unused_mut)]
fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Simulation", "j")
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
    let o = OrbitalBody::new((400., 400.), 20.);
    qt.insert(o);

    let simulation = Simulation::new(&mut ctx, qt);
    event::run(ctx, event_loop, simulation);
}
