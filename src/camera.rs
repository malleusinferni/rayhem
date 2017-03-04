use specs::{Join, RunArg, System};

use engine::Ctx;

use component::*;
use resource::*;

pub struct MoveCamera;

impl System<Ctx> for MoveCamera {
    fn run(&mut self, arg: RunArg, ctx: Ctx) {
        let (mut camera, pos, player) = arg.fetch(|world| {
            (world.write_resource::<Camera3D>(),
            world.read::<Pos3D>(),
            world.read::<IsPlayer>())
        });

        for (_, &Pos3D(ref pos, ref yaw)) in (&player, &pos).iter() {
            camera.pos = *pos;
            camera.yaw = *yaw;

            break;
        }
    }
}
