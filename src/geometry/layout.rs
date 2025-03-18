use super::*;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

use super::Shape;

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

// pub const ORIENTATION_FLAT: Orientation = Orientation {
//     f0: 3.0 / 2.0,
//     f1: 0.0,
//     f2: SQRT_3 / 2.0,
//     f3: SQRT_3,
//     b0: 2.0 / 3.0,
//     b1: 0.0,
//     b2: -1.0 / 3.0,
//     b3: SQRT_3 / 3.0,
//     start_angle: 0.0,
// };

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

// #[repr(u8)]
// #[derive(Debug, Clone, Copy)]
// pub enum LayoutType {
//     FLAT = 0,
//     POINTY = 1,
// }

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    orientation: Orientation,
    size: Pxl,
    origin: Pxl,
}

impl Layout {
    pub const fn new(orientation: Orientation, size: Pxl, origin: Pxl) -> Self {
        Self {
            orientation,
            size,
            origin,
        }
    }

    pub fn get_origin(&self) -> Pxl {
        self.origin
    }

    pub fn get_size(&self) -> Pxl {
        self.size
    }

    pub fn set_size(&mut self, size: Pxl) {
        self.size = size;
    }

    pub fn set_origin(&mut self, origin: Pxl) {
        self.origin = origin;
    }

    pub fn hex_to_pxl(&self, h: IHex) -> Pxl {
        self.fhex_to_point(h.into())
    }

    pub fn fhex_to_point(&self, h: FHex) -> Pxl {
        let m = &self.orientation;
        let x = (m.f0 * h.q() + m.f1 * h.r()) * self.size.x();
        let y = (m.f2 * h.q() + m.f3 * h.r()) * self.size.y();
        Pxl(x, y) + self.origin
    }

    pub fn point_to_fhex(&self, p: Pxl) -> FHex {
        let l = self;
        let m = l.orientation;
        let pt = (p - self.origin) / l.size;
        let q = m.b0 * pt.x() + m.b1 * pt.y();
        let r = m.b2 * pt.x() + m.b3 * pt.y();
        FHex(q, r)
    }

    pub fn point_to_hex(&self, p: Pxl) -> IHex {
        self.point_to_fhex(p).round()
    }

    pub fn hex_corner_offset(&mut self, corner: usize) -> Pxl {
        assert!(corner < 6);
        let angle =
            2.0 * core::f32::consts::PI * (self.orientation.start_angle + corner as f32) / 6.0;
        let (s, c) = angle.sin_cos();
        Pxl(self.size.x() * c, self.size.y() * s)
    }

    pub fn hex_centers<T: Shape>(&mut self, hexes: T, centers: &mut [Pxl]) {
        assert!(centers.len() >= hexes.area());
        hexes
            .hex_iter()
            .enumerate()
            .for_each(|(i, h)| centers[i] = self.hex_to_pxl(h));
    }

    pub fn grid_corners(&mut self, centers: &[Pxl], corners: &mut [Pxl]) {
        assert!(corners.len() >= centers.len() * 6);
        centers.iter().enumerate().for_each(|(i, c)| {
            for j in 0..6 {
                let off = self.hex_corner_offset(j);
                corners[i * 6 + j] = *c + off;
            }
        });
    }
}

pub const PXL_0: Pxl = Pxl(0., 0.);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pxl(pub f32, pub f32);

impl Pxl {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
}

impl Add for Pxl {
    type Output = Pxl;

    fn add(self, rhs: Self) -> Self::Output {
        Pxl(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pxl {
    type Output = Pxl;

    fn sub(self, rhs: Self) -> Self::Output {
        Pxl(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Pxl {
    type Output = Pxl;

    fn mul(self, rhs: Self) -> Self::Output {
        Pxl(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Pxl {
    type Output = Pxl;

    fn div(self, rhs: Self) -> Self::Output {
        Pxl(self.0 / rhs.0, self.1 / rhs.1)
    }
}
