use core::ops::Range;

use crate::hex::Hex;

// A rectangular shape of pointy top hex tiles.
pub struct HexRect {
    q0: i32,
    r0: i32,
    width: i32,
    height: i32,
}

impl HexRect {
    pub fn new(a: Hex, b: Hex) -> Self {
        let (aq, ar, aoff) = HexRect::offset(a);
        let (bq, br, boff) = HexRect::offset(b);
        let min_off = aoff.min(boff);
        let q0 = if aq < bq { aq - min_off } else { bq - min_off };
        let r0 = ar.min(br);
        let width = (aq - bq).abs() + 1;
        let height = (ar - br).abs() + 1;
        Self {
            q0,
            r0,
            width,
            height,
        }
    }

    pub fn qr0(&self) -> (i32, i32) {
        (self.q0, self.r0)
    }

    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn top_left(&self) -> Hex {
        Hex::ax(self.q0, self.r0)
    }

    pub fn top_right(&self) -> Hex {
        Hex::ax(self.q0 + self.width - 1, self.r0)
    }

    pub fn bot_left(&self) -> Hex {
        Hex::ax(self.q0 - (self.height - 1) / 2, self.r0 + self.height - 1)
    }

    pub fn bot_right(&self) -> Hex {
        Hex::ax(
            self.q0 + (self.width - 1) - (self.height - 1) / 2,
            self.r0 + (self.height - 1),
        )
    }

    pub fn corners(&self) -> [Hex; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bot_left(),
            self.bot_right(),
        ]
    }

    pub fn contains(&self, h: Hex) -> bool {
        let (q, r, _) = HexRect::offset(h);
        self.q_range().contains(&q) && self.r_range().contains(&r)
    }

    fn offset(h: Hex) -> (i32, i32, i32) {
        let off = h.r() / 2;
        (h.q() + off, h.r(), off)
    }

    pub fn qr_off(&self, h: Hex) -> Option<(i32, i32)> {
        let (q, r, _) = HexRect::offset(h);
        if self.q_range().contains(&q) && self.r_range().contains(&r) {
            Some((q - self.q0, r - self.r0))
        } else {
            None
        }
    }

    pub fn hex(&self, q_off: i32, r_off: i32) -> Option<Hex> {
        if (0..self.width).contains(&q_off) && (0..self.height).contains(&r_off) {
            let r = self.r0 + r_off;
            let off = r / 2;
            let q = self.q0 + q_off + off;
            Some(Hex::ax(q, r))
        } else {
            None
        }
    }

    pub fn hex_unchecked(&self, row: usize, col: usize) -> Hex {
        let rowi = row as i32;
        let coli = col as i32;
        let r = self.r0 + rowi;
        let q_off = r / 2;
        let q = self.q0 + coli - q_off;
        Hex::ax(q, r)
    }

    pub fn row_col_unchecked(&self, h: Hex) -> (usize, usize) {
        let (q, r, _) = HexRect::offset(h);
        ((q - self.q0) as usize, (r - self.r0) as usize)
    }

    fn q_range(&self) -> Range<i32> {
        self.q0..self.q0 + self.width
    }

    fn r_range(&self) -> Range<i32> {
        self.r0..self.r0 + self.height
    }
}

#[cfg(test)]
mod test {
    use crate::hex::Hex;

    use super::HexRect;

    #[test]
    fn rect_qr0_size() {
        let tl = Hex::ax(0, 0);
        let br = Hex::ax(3, 6);
        for (a, b) in [(tl, br), (br, tl)] {
            let r = HexRect::new(a, b);
            assert_eq!((0, 0), r.qr0());
            assert_eq!((7, 7), r.size());
        }
    }

    #[test]
    fn rect_corners() {
        let want = [(0, 3), (4, 3), (-1, 5), (3, 5)].map(|(q, r)| Hex::ax(q, r));
        for (i, j) in [(0, 3), (3, 0), (1, 2), (2, 1)] {
            let a = want[i];
            let b = want[j];
            let r = HexRect::new(a, b);
            assert_eq!(&want, &r.corners(), "a={:?}, b={:?}", a, b);
        }
    }
}
