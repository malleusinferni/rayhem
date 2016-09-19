use cgmath::Angle;

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
            let turn_speed = Radf::new((1.0f32).to_radians());
            match ctx.turning {
                Turning::Left => pos.1 += turn_speed,
                Turning::Right => pos.1 -= turn_speed,
                _ => (),
            }

            pos.1 = pos.1.normalize();

            if ctx.walking {
                pos.advance();
            }
        }
    }
}

impl Pos3D {
    fn advance(&mut self) {
        let angle = self.1;
        let distance = 3.0 / 60.0;
        let (sin, cos) = angle.sin_cos();
        let offset = Vec3f::new(distance * cos, distance * sin, 0.0);
        self.0 += offset;
    }
}
