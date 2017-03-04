use specs::*;

use geom::*;
use component::TextureID;

#[derive(Clone, Debug)]
pub struct Pos3D(pub Vec3f, pub Radf);

#[derive(Clone, Debug)]
pub struct Vel3D(pub Vec3f);

impl Component for Pos3D { type Storage = VecStorage<Pos3D>; }

impl Component for Vel3D { type Storage = VecStorage<Vel3D>; }

static DEBUG_MAP: &'static str = {
    r#"XXXXXXXX
       X......X
       X......X
       X..a...X
       X....b.X
       X.c....X
       X......X
       XXXXXXXX"#
};

#[derive(Clone, Debug)]
pub struct LevelMap {
    pub chunks: Vec<Chunk>,
    pub grid_size: f32,
}

impl LevelMap {
    pub fn new() -> Self {
        let mut sectors = [[Sector::default(); 8]; 8];
        let mut x = 0;
        let mut y = 7;
        for c in DEBUG_MAP.chars() {
            sectors[y][x] = match c {
                '.' => Sector {
                    floor_height: 0,
                    texid: TextureID(0),
                },

                'X' => Sector {
                    floor_height: 1,
                    texid: TextureID(1),
                },

                'a' => Sector {
                    floor_height: 1,
                    texid: TextureID(2),
                },

                'b' => Sector {
                    floor_height: 1,
                    texid: TextureID(3),
                },

                'c' => Sector {
                    floor_height: 1,
                    texid: TextureID(4),
                },

                '\n' => {
                    x = 0;
                    y -= 1;
                    continue;
                },

                _ => continue,
            };

            x += 1;
        }

        LevelMap {
            chunks: vec![Chunk { sectors: sectors }],
            grid_size: 3.0,
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub sectors: [[Sector; 8]; 8]
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sector {
    pub floor_height: i16,
    pub texid: TextureID,
}

impl Default for Sector {
    fn default() -> Self {
        Sector {
            floor_height: 0,
            texid: TextureID::default(),
        }
    }
}

impl Pos3D {
    pub fn new(x: f32, y: f32, z: f32, deg: f32) -> Self {
        let pos = Vec3f::new(x, y, z);
        let yaw = Rad(deg.to_radians());
        Pos3D(pos, yaw)
    }
}

impl Vel3D {
    pub fn new() -> Self {
        Vel3D(Vec3f::new(0.0, 0.0, 0.0))
    }
}
