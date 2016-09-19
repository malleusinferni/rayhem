use specs::{Join, RunArg, System};

use component::*;
use engine::*;

#[derive(Copy, Clone, Debug)]
pub enum Turning {
    Left,
    Right,
    Straight,
}

pub struct MovePlayer;

impl System<Ctx> for MovePlayer {
    fn run(&mut self, arg: RunArg, ctx: Ctx) {
        let (player, mut pos) = arg.fetch(|world| {
            (world.read::<IsPlayer>(), world.write::<Pos3D>())
        });

        for (_, pos) in (&player, &mut pos).iter() {
            let turn_speed = Radf::new((0.5f32).to_radians());
            match ctx.turning {
                Turning::Left => pos.1 -= turn_speed,
                Turning::Right => pos.1 += turn_speed,
                _ => (),
            }
        }
    }
}
