use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use specs::{Component, HashMapStorage, Join, RunArg, System, VecStorage};
use specs::{Planner, World};

use engine::Ctx;

use geom::*;

use movement::*;

use map::*;

pub struct Camera3D {
    pub pos: Vec3f,
    pub dim: Vec2u,
    pub yaw: Radf,
    pub pitch: Radf,
}

#[derive(Clone, Debug)]
pub struct Sprite3D {
    // Empty for now
}

#[derive(Copy, Clone, Debug)]
pub struct Billboard {
    pub dst_pos: Vec2i,
    pub src_pos: Vec2i,
    pub size: Vec2u,
    pub depth: f32,
    pub texid: TextureID,
}

impl Component for Sprite3D { type Storage = VecStorage<Sprite3D>; }
impl Component for Billboard { type Storage = HashMapStorage<Billboard>; }

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextureID(pub u8);

struct DisplayList {
    bg: Color,
    walls: Vec<WallSlice>,
}

struct WallSlice {
    texid: TextureID,
    camera_z: f32,
    high_y: i16,
    low_y: i16,
    x: u32,
}

pub struct MoveCamera;

pub struct Draw {
    resolution: Vec2u,
}

pub struct Handler<'r> {
    textures: HashMap<TextureID, Color>,
    renderer: Renderer<'r>,
    resolution: Vec2u,
}

pub fn init<'r>(planner: &mut Planner<Ctx>, mut renderer: Renderer<'r>) -> Handler<'r> {
    let (width, height) = renderer.window().unwrap().size();

    let desired_res = {
        let divisor = gcd(width, height);

        // TODO: Try multiplying different factors together
        // and dividing the original resolution by the result
        // until you get an area within the desired range
        Vec2u::from(match (width / divisor, height / divisor) {
            (16, 9) => (400, 225),
            (8, 5) => (720, 450),
            (4, 3) => (320, 240),
            _ => unimplemented!(),
        })
    };

    renderer.set_logical_size(desired_res.x, desired_res.y)
        .unwrap();

    println!("Using resolution: {:?}", desired_res);

    let mut textures = HashMap::new();
    textures.insert(TextureID(0), Color::RGB(0x00, 0x00, 0x00));
    textures.insert(TextureID(1), Color::RGB(0x7f, 0x3f, 0x1f));
    textures.insert(TextureID(2), Color::RGB(0x00, 0xbf, 0x1f));
    textures.insert(TextureID(3), Color::RGB(0xbf, 0xbf, 0x1f));
    textures.insert(TextureID(4), Color::RGB(0xbf, 0xbf, 0xbf));

    let handler = Handler {
        renderer: renderer,
        textures: textures,
        resolution: desired_res,
    };

    let draw = Draw {
        resolution: desired_res,
    };

    planner.add_system(MoveCamera{}, "display::MoveCamera", 2);
    planner.add_system(draw, "display::Draw", 1);

    let world = planner.mut_world();
    world.register::<Sprite3D>();
    world.register::<Billboard>();
    world.add_resource(Camera3D::new(desired_res));
    world.add_resource(DisplayList::new(desired_res));

    handler
}

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

impl System<Ctx> for Draw {
    fn run(&mut self, arg: RunArg, _ctx: Ctx) {
        let (mut manifest, billboards, camera, level) = arg.fetch(|world| {
            (world.write_resource::<DisplayList>(),
            world.read::<Billboard>(),
            world.read_resource::<Camera3D>(),
            world.read_resource::<LevelMap>())
        });

        let player_xy = camera.pos.truncate(); // Vec3f to Vec2f

        let hf = camera.dim.y as f32 / 8.0;

        for (x, ray) in camera.scatter_rays() {
            let mut prev = match level.sector_to_draw(player_xy) {
                Some(sector) => sector,
                None => continue,
            };

            for hit in ray.cast(level.grid_size) {
                use geom::dda::RayHit;

                let hit: RayHit = hit;

                if hit.toi > 1000.0 { break; }

                let next: Sector = {
                    let spot = hit.poi + match hit.normal {
                        Cardinal::North => Vec2f::new(0.0, 0.5),
                        Cardinal::South => Vec2f::new(0.0, -0.5),
                        Cardinal::East => Vec2f::new(0.5, 0.0),
                        Cardinal::West => Vec2f::new(-0.5, 0.0),
                    };

                    match level.sector_to_draw(spot) {
                        Some(sector) => sector,
                        None => break,
                    }
                };

                if next.floor_height <= prev.floor_height { continue; }

                // FIXME: Correct projection
                let z = hit.toi;
                let sector_height = next.floor_height as f32 * hf;

                // Assume current floor height is 0
                let wall_height = (sector_height / z) as i16;

                manifest.walls.push(WallSlice {
                    texid: next.texid,
                    camera_z: z,
                    high_y: wall_height,
                    low_y: -wall_height,
                    x: x,
                });

                break;
            }
        }
    }
}

impl<'r> Handler<'r> {
    pub fn draw(&mut self, world: &mut World) {
        let mut manifest = world.write_resource::<DisplayList>();

        //manifest.billboards.sort_by_key(|b| b.dst_pos.x);

        self.renderer.set_draw_color(manifest.bg);
        self.renderer.clear();

        let camera_y = (self.resolution.y / 2) as i32;

        for wall in manifest.walls.drain(..) {
            let x = wall.x as i32;

            let color = match self.textures.get(&wall.texid) {
                Some(c) => c, None => continue,
            };

            self.renderer.set_draw_color(*color);
            let high_y = camera_y - wall.high_y as i32;
            let low_y = camera_y - wall.low_y as i32;
            let width = 1;
            let height = (low_y - high_y).abs() as u32;

            let screen_rect = Rect::new(x, high_y, width, height);
            self.renderer.draw_rect(screen_rect).unwrap();
        }

        //for billboard in manifest.billboards.drain(..) {
        //    self.renderer.set_draw_color(billboard.texid.0);
        //    let dst_rect = billboard.screen_rect(camera_y);
        //    self.renderer.fill_rect(dst_rect).unwrap();
        //}

        self.renderer.present();
    }
}

impl DisplayList {
    fn new(resolution: Vec2u) -> Self {
        DisplayList {
            bg: Color::RGB(0x3f, 0x7f, 0xff),
            walls: Vec::with_capacity(resolution.x as usize),
        }
    }
}

impl Billboard {
    fn dst_rect(self) -> Rect {
        let (x, y) = self.dst_pos.into();
        let (w, h) = self.size.into();
        Rect::new(x, y, w, h)
    }

    fn screen_rect(self, camera_y: i32) -> Rect {
        let (x, y) = self.dst_pos.into();
        let (w, h) = self.size.into();
        let y = camera_y - (y + h as i32);
        Rect::new(x, y, w, h)
    }
}

impl Default for TextureID {
    fn default() -> Self {
        TextureID(0)
    }
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

    fn scatter_rays(&self) -> XRayIter {
        use cgmath::prelude::*;

        let rot = Rot2f::from_angle(self.yaw);
        let plane_len = self.dim.x as f32 / self.dim.y as f32;

        XRayIter {
            x: 0,
            width: self.dim.x,
            src: self.pos.truncate().cast(),
            dir: rot.rotate_vector(Vec2f::new(1.0, 0.0)),
            right: rot.rotate_vector(Vec2f::new(0.0, -plane_len / 2.0)),
        }
    }
}

struct XRayIter {
    x: u32,
    width: u32,
    src: Vec2f,
    dir: Vec2f,
    right: Vec2f,
}

impl Iterator for XRayIter {
    type Item = (u32, Ray2f);

    fn next(&mut self) -> Option<(u32, Ray2f)> {
        if self.x >= self.width { return None; }

        let x = self.x;
        let width = self.width as f32;
        self.x += 1;
        let frac_of_view = 2.0 * x as f32 / width - 1.0;
        let dir = self.dir + self.right * frac_of_view;
        Some((x, Ray2f::new(self.src, dir)))
    }
}

impl LevelMap {
    #[inline]
    fn sector_to_draw(&self, poi: Vec2f) -> Option<Sector> {
        let coords: Vec2i = (poi / self.grid_size).cast();
        // FIXME: Subtract chunk root

        if coords.x < 0 || 8 <= coords.x { return None; }
        if coords.y < 0 || 8 <= coords.y { return None; }

        let y = coords.y as usize;
        let x = coords.x as usize;

        Some(self.chunks[0].sectors[y][x])
    }
}
