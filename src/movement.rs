use specs::{Join, RunArg, System};

use engine::*;

use component::*;

use map::*;

pub struct ApplyVelocity;

impl System<Ctx> for ApplyVelocity {
    fn run(&mut self, arg: RunArg, ctx: Ctx) {
        let (mut pos, mut vel) = arg.fetch(|world| {
            (world.write::<Pos3D>(), world.write::<Vel3D>())
        });

        for (pos, vel) in (&mut pos, &mut vel).iter() {
            pos.0 += vel.0;
            //vel.0 = Vec3f::new();
        }
    }
}
