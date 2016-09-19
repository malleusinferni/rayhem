use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use specs::{Join, RunArg, System};

use engine::Ctx;

use component::*;
use resource::*;

struct DisplayList {
    bg: Color,
    walls: Vec<WallSlice>,
}

struct WallSlice {
    texid: TextureID,
    camera_z: f32,
    high_y: i16,
    low_y: i16,
}

pub struct DisplaySys {
    inbox: Receiver<DisplayList>,
    outbox: Sender<DisplayList>,
    resolution: Vec2u,
}

pub struct DisplayAgent<'r> {
    inbox: Receiver<DisplayList>,
    outbox: Sender<DisplayList>,
    textures: HashMap<TextureID, Color>,
    renderer: Renderer<'r>,
    resolution: Vec2u,
}

impl DisplaySys {
    pub fn new<'r>(mut renderer: Renderer<'r>) -> (DisplayAgent<'r>, Self) {
        let (agent_out, sys_in) = mpsc::channel();
        let (sys_out, agent_in) = mpsc::channel();

        let (width, height) = renderer.window().unwrap().size();

        let desired_res = {
            let divisor = gcd(width, height);

            // TODO: Try multiplying different factors together
            // and dividing the original resolution by the result
            // until you get an area within the desired range
            Vec2u::from(match (width / divisor, height / divisor) {
                (16, 9) => (400, 225),
                (8, 5) => (360, 225),
                (4, 3) => (320, 240),
                _ => unimplemented!(),
            })
        };

        renderer.set_logical_size(desired_res.x, desired_res.y)
            .unwrap();

        println!("Using resolution: {:?}", desired_res);

        let mut textures = HashMap::new();
        textures.insert(TextureID(0), Color::RGB(0x00, 0x7f, 0x1f));
        textures.insert(TextureID(1), Color::RGB(0x7f, 0x3f, 0x1f));
        textures.insert(TextureID(2), Color::RGB(0x3f, 0x3f, 0x3f));

        let agent = DisplayAgent {
            inbox: agent_in,
            outbox: agent_out,
            renderer: renderer,
            textures: textures,
            resolution: desired_res,
        };

        for _ in 0 .. 2 {
            agent.outbox.send(DisplayList::new(desired_res)).unwrap();
        }

        (agent, DisplaySys {
            inbox: sys_in,
            outbox: sys_out,
            resolution: desired_res,
        })
    }
}

impl System<Ctx> for DisplaySys {
    fn run(&mut self, arg: RunArg, _ctx: Ctx) {
        let (billboards, camera, level) = arg.fetch(|world| {
            (world.read::<Billboard>(),
            world.read_resource::<Camera3D>(),
            world.read_resource::<LevelMap>())
        });

        let mut manifest: DisplayList = match self.inbox.try_recv() {
            Ok(m) => m,
            Err(_) => return,
        };

        let player_xy = camera.pos.truncate();

        for (x, ray) in camera.scatter_rays() {
            //let mut prev = match level.sector_to_draw(player_xy + ray.dir) {
            //    Some(sector) => sector,
            //    None => continue,
            //};

            for hit in ray.cast(level.grid_size) {
                use geom::dda::RayHit;

                let hit: RayHit = hit;

                if hit.toi > 1000.0 { break; }

                let next: Sector = match level.sector_to_draw(&hit) {
                    Some(sector) => sector,
                    None => break,
                };

                if next.floor_height <= 0 { continue; }

                // FIXME: Correct projection
                let z = hit.toi;

                // Assume current floor height is 0
                let wall_height = (next.floor_height as f32 / z) as i16;

                manifest.walls.push(WallSlice {
                    texid: next.texid,
                    camera_z: z,
                    high_y: wall_height,
                    low_y: -wall_height,
                });
            }
        }

        let _ = self.outbox.send(manifest).map_err(|e| println!("{}", e));
    }
}

impl<'r> DisplayAgent<'r> {
    pub fn draw(&mut self) {
        let mut manifest: DisplayList = match self.inbox.recv() {
            Ok(m) => m,
            Err(_) => return,
        };

        //manifest.billboards.sort_by_key(|b| b.dst_pos.x);

        self.renderer.set_draw_color(manifest.bg);
        self.renderer.clear();

        let camera_y = (self.resolution.y / 2) as i32;

        for (x, wall) in manifest.walls.drain(..).enumerate() {
            let x = x as i32;

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

        let _ = self.outbox.send(manifest).map_err(|e| println!("{}", e));
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

impl Camera3D {
    fn scatter_rays(&self) -> XRayIter {
        use cgmath::prelude::*;

        let rot = Rot2f::from_angle(self.yaw);
        let plane_len = self.dim.x as f32 / self.dim.y as f32;

        XRayIter {
            x: 0,
            width: self.dim.x,
            src: self.pos.truncate().cast(),
            dir: rot.rotate_vector(Vec2f::new(1.0, 0.0)),
            right: rot.rotate_vector(Vec2f::new(0.0, plane_len / 2.0)),
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

use geom::dda::RayHit;

impl LevelMap {
    #[inline]
    fn sector_to_draw(&self, hit: &RayHit) -> Option<Sector> {
        let offset = match hit.normal {
            Cardinal::North => Vec2f::new(0.0, 0.5),
            Cardinal::South => Vec2f::new(0.0, -0.5),
            Cardinal::East => Vec2f::new(0.5, 0.0),
            Cardinal::West => Vec2f::new(-0.5, 0.0),
        };

        let poi: Vec2f = hit.poi + offset;

        let coords: Vec2i = (poi / self.grid_size).cast();
        // FIXME: Subtract chunk root

        if coords.x < 0 || 8 <= coords.x { return None; }
        if coords.y < 0 || 8 <= coords.y { return None; }

        let y = coords.y as usize;
        let x = coords.x as usize;

        Some(self.chunks[0].sectors[y][x])
    }
}
