use crate::geometry::{HexDir, IHex, Shape, IHEX_0};
use std::iter::once;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    h0: IHex,
    h1: IHex,
    t: LineType,
}

#[derive(Clone, Copy, Debug)]
enum LineType {
    G, // generic
    Q, // q-axis
    R, // r-axis
    S, // s-axis
    V, // vertical for pointy-top
}

use LineType::*;

impl Line {
    pub fn new(h0: IHex, h1: IHex) -> Self {
        let t;
        if h0.q() == h1.q() {
            t = Q;
        } else if h0.r() == h1.r() {
            t = R;
        } else if h0.s() == h1.s() {
            t = S;
        } else {
            t = G;
        }
        Line { h0, h1, t }
    }
}

impl Shape for Line {
    fn center(&self) -> IHex {
        self.h0
    }

    fn area(&self) -> usize {
        match self.t {
            Q => (self.h0.r() - self.h1.r()).unsigned_abs() as usize,
            R | S => (self.h0.q() - self.h1.q()).unsigned_abs() as usize,
            _ => self.h0.distance(self.h1) as usize,
        }
    }

    fn hex_iter(&self) -> impl Iterator<Item = IHex> {
        once(self.h0)
    }

    fn hex_by_ind(&self, ind: usize) -> Option<IHex> {
        if ind >= self.area() {
            None
        } else {
            let diff = self.h1 - self.h0;
            match self.t {
                G => Some(self.h0.nlerp(self.h1, ind)),
                Q => Some(IHex(
                    self.h0.q(),
                    self.h0.r() + ind as i32 * diff.r().signum(),
                )),
                R => Some(IHex(
                    self.h0.q() + ind as i32 * diff.q().signum(),
                    self.h0.r(),
                )),
                S => {
                    let q = self.h0.q() + ind as i32 * diff.q().signum();
                    Some(IHex(q, -q))
                }
                V => {
                    let q_off = (ind / 2) as i32 * diff.q().signum();
                    let r = ind as i32 * diff.r().signum();
                    Some(IHex(self.h0.q() - q_off, self.h0.r() + r))
                }
            }
        }
    }

    fn ind_by_hex(&self, hex: IHex) -> Option<usize> {
        None
    }

    fn contains(&self, hex: IHex) -> bool {
        false
    }

    fn move_to(&self, hex: IHex) -> Self {
        Line {
            h0: hex,
            h1: self.h1 + hex - self.h0,
            t: self.t,
        }
    }

    fn all_neighbors(&self) -> impl Iterator<Item = IHex> {
        once(IHEX_0)
    }

    fn direction_neighbors(&self, d: HexDir) -> impl Iterator<Item = IHex> {
        once(IHEX_0)
    }
}


#[cfg(test)]
mod tests {

    use crate::geometry::line::{Line, LineType};
    use crate::geometry::{IHex, Shape};

    #[test]
    fn line_center() {
        let l = Line::new(IHex(1, 0), IHex(5, 0));
        assert_eq!(l.area(), 4);
    }

}
