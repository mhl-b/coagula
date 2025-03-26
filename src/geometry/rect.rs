use std::iter::once;
use crate::geometry::{HexDir, IHex, Shape, IHEX_0};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    h0: IHex,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(h0: IHex, w: i32, h: i32) -> Self {
        Rect { h0, w, h }
    }
}

#[inline]
const fn h0_to_hc(h0: IHex, w: i32, h: i32) -> IHex {
    let q_off = h / 2;
    IHex(h0.q() + w / 2 - q_off, h0.r() + h / 2)
}

#[inline]
const fn hc_to_h0(hc: IHex, w: i32, h: i32) -> IHex {
    let q_off = h / 2;
    IHex(hc.q() - w / 2 + q_off, hc.r() - h / 2)
}

impl Shape for Rect {
    fn center(&self) -> IHex {
        h0_to_hc(self.h0, self.w, self.h)
    }

    fn area(&self) -> usize {
        self.w as usize * self.h as usize
    }

    fn hex_iter(&self) -> impl Iterator<Item = IHex> {
        RectIter {
            rect: *self,
            i: 0,
        }
    }

    fn hex_by_ind(&self, ind: usize) -> Option<IHex> {
        if ind >= self.area() {
            None
        } else {
            let r = ind as i32 / self.w;
            let q = ind as i32 % self.w - r / 2;
            Some(IHex(q + self.h0.q(), r + self.h0.r()))
        }
    }

    fn ind_by_hex(&self, hex: IHex) -> Option<usize> {
        let off = hex - self.h0;
        let r_off = off.r();
        let q_off = off.q() + r_off / 2;
        if q_off >= 0 && q_off < self.w && r_off >= 0 && r_off < self.h {
            Some((q_off + r_off * self.w) as usize)
        } else {
            None
        }
    }

    fn contains(&self, hex: IHex) -> bool {
        self.ind_by_hex(hex).is_some()
    }

    fn move_to(&self, hex: IHex) -> Self {
        Rect::new(hc_to_h0(hex, self.w, self.h), self.w, self.h)
    }

    fn all_neighbors(&self) -> impl Iterator<Item = IHex> {
        once(IHEX_0)
    }

    fn direction_neighbors(&self, d: HexDir) -> impl Iterator<Item = IHex> {
        once(IHEX_0)
    }
}

struct RectNeighIter {
    rect: Rect,
    state: RectNeighSideIter,
}

impl RectNeighIter {
    fn new(rect: Rect) -> Self {
        RectNeighIter {
            rect,
            state: RectNeighSideIter::Top { i: 0 },
        }
    }
}

enum RectNeighSideIter {
    Top { i: usize },
    Right { i: usize },
    Bottom { i: usize },
    Left { i: usize },
}
//
// impl Iterator for RectNeighSideIter {
//     type Item = IHex;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl Iterator for RectNeighIter {
//     type Item = IHex;
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

struct RectIter {
    rect: Rect,
    i: usize,
}

impl Iterator for RectIter {
    type Item = IHex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rect.hex_by_ind(self.i) {
            Some(h) => {
                self.i += 1;
                Some(h)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::rect::Rect;
    use crate::geometry::{IHex, Shape};

    #[test]
    fn hex_by_ind() {
        let r = Rect::new(IHex(0, 0), 3, 3);
        assert_eq!(r.hex_by_ind(0), Some(IHex(0, 0)));
        assert_eq!(r.hex_by_ind(5), Some(IHex(2, 1)));
        assert_eq!(r.hex_by_ind(8), Some(IHex(1,2)));
    }
}
