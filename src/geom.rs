use cgmath;

pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    use std::mem::swap;

    if b > a { swap(&mut a, &mut b); }
    loop {
        if b == 0 || a == b { return a; }
        a %= b;
        swap(&mut a, &mut b);
    }
}

pub type Vec2u = cgmath::Vector2<u32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec2f = cgmath::Vector2<f32>;
pub type Vec3f = cgmath::Vector3<f32>;

pub type Rot2f = cgmath::Basis2<f32>;

pub type Radf = cgmath::Rad<f32>;

#[derive(Copy, Clone)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Ray2f {
    pub src: Vec2f,
    pub dir: Vec2f,
}

impl Ray2f {
    pub fn new(src: Vec2f, dir: Vec2f) -> Self {
        Ray2f { src: src, dir: dir, }
    }

    pub fn angle(self) -> f32 {
        self.dir.y.atan2(self.dir.x)
    }
}

pub mod dda {
    use super::*;

    impl Ray2f {
        pub fn cast(self, grid_size: f32) -> DDA {
            DDA {
                h: horizontal(self.src, self.dir, grid_size),
                v: vertical(self.src, self.dir, grid_size),
            }
        }
    }

    fn horizontal(src: Vec2f, dir: Vec2f, grid_size: f32) -> Iter {
        use cgmath::prelude::*;

        let slope = dir.y / dir.x;

        let first_x = {
            let mut first = (src.x / grid_size).floor();
            if dir.x > 0.0 { first += 1.0; }
            first * grid_size
        };

        let delta_x = dir.x.signum() * grid_size;
        let delta_poi = Vec2f::new(delta_x, delta_x * slope);
        let delta_first = delta_poi * ((first_x - src.x) / delta_x);

        Iter {
            poi: src + delta_first,
            toi: delta_first.magnitude(),
            delta_poi: delta_poi,
            delta_toi: delta_poi.magnitude(),
            normal: {
                use super::Cardinal::*;
                if dir.x > 0.0 { East } else { West }
            },
        }
    }

    fn vertical(src: Vec2f, dir: Vec2f, grid_size: f32) -> Iter {
        let flip = |v: Vec2f| { Vec2f::new(v.y, v.x) };
        let h = horizontal(flip(src), flip(dir), grid_size);

        Iter {
            toi: h.toi,
            delta_toi: h.delta_toi,

            poi: flip(h.poi),
            delta_poi: flip(h.delta_poi),

            normal: match h.normal {
                Cardinal::East => Cardinal::North,
                Cardinal::West => Cardinal::South,
                _ => unreachable!(),
            },
        }
    }

    pub struct DDA {
        h: Iter,
        v: Iter,
    }

    pub struct RayHit {
        pub poi: Vec2f,
        pub toi: f32,
        pub normal: Cardinal,
    }

    pub struct Iter {
        poi: Vec2f,
        toi: f32,
        delta_poi: Vec2f,
        delta_toi: f32,
        normal: Cardinal,
    }

    impl Iterator for DDA {
        type Item = RayHit;

        fn next(&mut self) -> Option<Self::Item> {
            if self.h.toi < self.v.toi { self.h.next() } else { self.v.next() }
        }
    }

    impl Iterator for Iter {
        type Item = RayHit;

        fn next(&mut self) -> Option<Self::Item> {
            let hit = RayHit {
                poi: self.poi,
                toi: self.toi,
                normal: self.normal,
            };

            self.poi += self.delta_poi;
            self.toi += self.delta_toi;

            Some(hit)
        }
    }

    #[test]
    fn raycast_example() {
        let sqrt2 = (2.0f32).sqrt();

        let ray = Ray2f {
            src: Vec2f::new(0.5, 0.5),
            dir: Vec2f::new(sqrt2, sqrt2),
        };

        let wanted = vec![
            Vec2f::new(2.0, 2.0),
            Vec2f::new(2.0, 2.0),
            Vec2f::new(4.0, 4.0),
            Vec2f::new(4.0, 4.0),
        ];

        let got = ray.cast(2.0).take(4).map(|hit: RayHit| {
            hit.poi
        }).collect::<Vec<_>>();

        assert_eq!(wanted, got);

        let ray2 = Ray2f {
            src: Vec2f::new(0.5, 0.5),
            dir: Vec2f::new(-sqrt2, -sqrt2),
        };

        let wanted2 = vec![
            Vec2f::new(0.0, 0.0),
            Vec2f::new(0.0, 0.0),
            Vec2f::new(-2.0, -2.0),
            Vec2f::new(-2.0, -2.0),
        ];

        let got2 = ray2.cast(2.0).take(4).map(|hit: RayHit| {
            hit.poi
        }).collect::<Vec<_>>();

        assert_eq!(wanted2, got2);
    }
}
