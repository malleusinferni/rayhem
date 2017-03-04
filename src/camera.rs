use specs::{Join, RunArg, System};

use engine::Ctx;

use component::*;

pub struct Camera3D {
    pub pos: Vec3f,
    pub dim: Vec2u,
    pub yaw: Radf,
    pub pitch: Radf,
}

impl Camera3D {
    pub fn new(dim: Vec2u) -> Self {
        Camera3D {
            dim: dim,
            pos: Vec3f::new(0.0, 0.0, 0.0),
            yaw: Rad(0.0),
            pitch: Rad(0.0),
        }
    }
}

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
