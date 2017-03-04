use specs::{Component, NullStorage};

pub use geom::*;

#[derive(Clone, Debug)]
pub struct Collider {
    pub height: f32,
    pub radius: f32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct IsPlayer;

impl Component for IsPlayer { type Storage = NullStorage<IsPlayer>; }
