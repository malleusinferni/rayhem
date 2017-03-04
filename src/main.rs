extern crate cgmath;
extern crate rand;
extern crate sdl2;
extern crate specs;
extern crate time;

pub mod geom;
pub mod engine;
pub mod input;
pub mod camera;
pub mod movement;
pub mod display;

pub mod component;
pub mod resource;

fn main() {
    let mut engine = engine::new();
    engine.run();
}
