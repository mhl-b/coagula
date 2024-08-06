use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Sub};

use crate::geometry::{FHex, fhex, Hex};

const SQRT_3: f32 = 1.7320509;

#[derive(Clone, Copy, PartialEq)]
pub struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
    start_angle: f32, // in multiples of 60°
}

impl Debug for Orientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.start_angle == 0.0 {
            f.write_str("flat")
        } else {
            f.write_str("pointy")
        }
    }
}

pub const ORIENTATION_FLAT: Orientation = Orientation {
    f0: 3.0 / 2.0,
    f1: 0.0,
    f2: SQRT_3 / 2.0,
    f3: SQRT_3,
    b0: 2.0 / 3.0,
    b1: 0.0,
    b2: -1.0 / 3.0,
    b3: SQRT_3 / 3.0,
    start_angle: 0.0,
};

pub const ORIENTATION_POINTY: Orientation = Orientation {
    f0: SQRT_3,
    f1: SQRT_3 / 2.0,
    f2: 0.0,
    f3: 3.0 / 2.0,
    b0: SQRT_3 / 3.0,
    b1: -1.0 / 3.0,
    b2: 0.0,
    b3: 2.0 / 3.0,
    start_angle: 0.5,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LayoutType {
    FLAT = 0,
    POINTY = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    orientation: Orientation,
    size: Point,
    origin: Point,

    corner_offset_cache: [Point; 6],
}


impl Layout {
    pub const fn new(orientation: Orientation, size: Point, origin: Point) -> Self {
        Self {
            orientation,
            size,
            origin,
            corner_offset_cache: [POINT0; 6],
        }
    }

    pub fn get_origin(&self) -> Point {
        self.origin
    }

    pub fn get_size(&self) -> Point {
        self.size
    }

    pub fn set_size(&mut self, size: Point) {
        self.size = size;
        self.corner_offset_cache = [POINT0; 6];
    }

    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    pub fn hex_to_point(&self, h: Hex) -> Point {
        self.fhex_to_point(h.tof())
    }

    pub fn fhex_to_point(&self, h: FHex) -> Point {
        let m = &self.orientation;
        let x = (m.f0 * h.q() + m.f1 * h.r()) * self.size.x();
        let y = (m.f2 * h.q() + m.f3 * h.r()) * self.size.y();
        point(x, y) + self.origin
    }

    pub fn point_to_fhex(&self, p: Point) -> FHex {
        let l = self;
        let m = l.orientation;
        let pt = (p - self.origin) / l.size;
        let q = m.b0 * pt.x() + m.b1 * pt.y();
        let r = m.b2 * pt.x() + m.b3 * pt.y();
        fhex(q, r)
    }

    pub fn point_to_hex(&self, p: Point) -> Hex {
        self.point_to_fhex(p).round()
    }

    pub fn hex_corner_offset(&mut self, corner: usize) -> Point {
        assert!(corner < 6);
        if self.corner_offset_cache[corner] != POINT0 {
            return self.corner_offset_cache[corner];
        }
        let angle = 2.0 * core::f32::consts::PI * (self.orientation.start_angle + corner as f32) / 6.0;
        let (s, c) = libm::sincosf(angle);
        let pt = point(self.size.x() * c, self.size.y() * s);
        self.corner_offset_cache[corner] = pt;
        pt
    }

    pub fn grid_centers(&mut self, hexes: &[Hex], centers: &mut [Point]) {
        assert!(centers.len() >= hexes.len());
        hexes.iter().enumerate().for_each(|(i, h)| { centers[i] = self.hex_to_point(*h) });
    }

    pub fn grid_corners(&mut self, centers: &[Point], corners: &mut [Point]) {
        assert!(corners.len() >= centers.len() * 6);
        centers.iter().enumerate().for_each(|(i, c)| {
            for j in 0..6 {
                let off = self.hex_corner_offset(j);
                corners[i * 6 + j] = *c + off;
            }
        });
    }
}

pub const POINT0: Point = point(0., 0.);

pub const fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        point(self.x * rhs.x, self.y * rhs.y)
    }
}


impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        point(self.x / rhs.x, self.y / rhs.y)
    }
}