use geom::*;
use component::TextureID;

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
