use super::*;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use sdl2::rect::FPoint;

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
    size: FPoint,
    origin: FPoint,
}

impl Layout {
    pub const fn new(orientation: Orientation, size: FPoint, origin: FPoint) -> Self {
        Self {
            orientation,
            size,
            origin,
        }
    }

    pub fn get_origin(&self) -> FPoint {
        self.origin
    }

    pub fn get_size(&self) -> FPoint {
        self.size
    }

    pub fn set_size(&mut self, size: FPoint) {
        self.size = size;
    }

    pub fn set_origin(&mut self, origin: FPoint) {
        self.origin = origin;
    }

    pub fn hex_to_pxl(&self, h: IHex) -> FPoint {
        self.fhex_to_point(h.into())
    }

    pub fn fhex_to_point(&self, h: FHex) -> FPoint {
        let m = &self.orientation;
        let x = (m.f0 * h.q() + m.f1 * h.r()) * self.size.x();
        let y = (m.f2 * h.q() + m.f3 * h.r()) * self.size.y();
        FPoint::new(x, y) + self.origin
    }

    pub fn point_to_fhex(&self, p: FPoint) -> FHex {
        let l = self;
        let m = l.orientation;
        let pt = p - self.origin;
        let ptl = FPoint::new(pt.x()/l.size.x(), pt.y()/l.size.y());
        let q = m.b0 * ptl.x() + m.b1 * ptl.y();
        let r = m.b2 * ptl.x() + m.b3 * ptl.y();
        FHex(q, r)
    }

    pub fn point_to_hex(&self, p: FPoint) -> IHex {
        self.point_to_fhex(p).round()
    }

    pub fn hex_corner_offset(&self, corner: usize) -> FPoint {
        assert!(corner < 6);
        let angle =
            2.0 * core::f32::consts::PI * (self.orientation.start_angle + corner as f32) / 6.0;
        let (s, c) = angle.sin_cos();
        FPoint::new(self.size.x() * c, self.size.y() * s)
    }

    pub fn hex_centers(&self, hexes: impl Iterator<Item = IHex>) -> impl Iterator<Item = FPoint> {
        hexes.map(|h| self.hex_to_pxl(h))
    }

    pub fn grid_corners(&self, c: FPoint) -> [FPoint; 6] {
        let mut corners = [FPoint::new(0., 0.); 6];
        (0..6).for_each(|i| {
            corners[i] = self.hex_corner_offset(i) + c;
        });
        corners
    }
}
