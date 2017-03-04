use specs::{Component, HashMapStorage, NullStorage, VecStorage};

use sdl2::pixels::Color;

pub use geom::*;

#[derive(Clone, Debug)]
pub struct Sprite3D {
    // Empty for now
}

#[derive(Clone, Debug)]
pub struct Collider {
    pub height: f32,
    pub radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Billboard {
    pub dst_pos: Vec2i,
    pub src_pos: Vec2i,
    pub size: Vec2u,
    pub depth: f32,
    pub texid: TextureID,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct IsPlayer;

impl Component for Sprite3D { type Storage = VecStorage<Sprite3D>; }
impl Component for Billboard { type Storage = HashMapStorage<Billboard>; }
impl Component for IsPlayer { type Storage = NullStorage<IsPlayer>; }

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextureID(pub u8);

impl Default for TextureID {
    fn default() -> Self {
        TextureID(0)
    }
}
