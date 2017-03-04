use cgmath::Angle;

use specs::{Join, RunArg, System};

use component::*;
use engine::*;

use map::*;

pub struct MovePlayer;

impl System<Ctx> for MovePlayer {
    fn run(&mut self, arg: RunArg, ctx: Ctx) {
        let (player, mut pos, mut vel) = arg.fetch(|world| {
            (world.read::<IsPlayer>(),
            world.write::<Pos3D>(),
            world.write::<Vel3D>())
        });

        for (_, pos, vel) in (&player, &mut pos, &mut vel).iter() {
            let turn_speed = Rad(ctx.turn_amount * ctx.dt as f32);

            pos.1 -= turn_speed;
            pos.1 = pos.1.normalize();

            let walk_speed = if ctx.walking {
                3.0 * ctx.dt as f32
            } else {
                0.0
            };

            let (sin, cos) = pos.1.sin_cos();
            vel.0 = Vec3f::new(walk_speed * cos, walk_speed * sin, 0.0);
        }
    }
}
