use hex::{HEX0, HEX_DIRECTIONS};

use super::{Direction, FHex, fhex, hex, Hex, Shape};

/// special case pointy top triangle with only 3 hexes
///   (1,0)
///(0,1) (1,1)
pub struct Tri {
    // right -> top -> left
    v: [Hex; 3],
}

impl Tri {
    pub fn new(h: Hex) -> Self {
        Self {
            v: [h, h - hex(0, 1), h - hex(1, 0)],
        }
    }
}

impl Shape for Tri {
    fn center(&self) -> Hex {
        self.v[0]
    }

    fn nearest_center(&self, hex: FHex) -> Hex {
        (hex + fhex(0.5, 0.5)).round()
    }

    fn area(&self) -> usize {
        3
    }

    fn hex_iter(&self) -> impl Iterator<Item=Hex> {
        self.v.into_iter()
    }

    fn hex_by_ind(&self, ind: usize) -> Option<Hex> {
        let hex = self.v.get(ind)?;
        Some(*hex)
    }

    fn ind_by_hex(&self, hex: Hex) -> Option<usize> {
        self.v.iter().position(|h| *h == hex)
    }

    fn contains(&self, hex: Hex) -> bool {
        self.v.contains(&hex)
    }

    fn move_to(&self, hex: Hex) -> Self {
        Tri::new(hex)
    }

    fn all_neighbors(&self) -> impl Iterator<Item=Hex> {
        TriAllIter::new(self.v[0])
    }

    fn direction_neighbors(&self, d: Direction) -> impl Iterator<Item=Hex> {
        assert!((0..6).contains(&d));
        let [top, left, right] = self.v;
        let adj = HEX_DIRECTIONS[d as usize];
        match d {
            0..=1 => [right + adj, top + adj].into_iter(),
            2..=3 => [top + adj, left + adj].into_iter(),
            4..=5 => [left + adj, right + adj].into_iter(),
            _ => [HEX0, HEX0].into_iter(),
        }
    }
}

struct TriAllIter {
    i: usize,
    hex: Hex,
}

impl TriAllIter {
    fn new(hex: Hex) -> Self {
        Self { i: 0, hex }
    }
}

impl Iterator for TriAllIter {
    type Item = Hex;

    fn next(&mut self) -> Option<Hex> {
        if self.i < 9 {
            self.hex += match self.i {
                0 => hex(1, 0),
                1..=2 => hex(0, -1),
                3 => hex(-1, 0),
                4..=5 => hex(-1, -1),
                6 => hex(0, 1),
                7..=8 => hex(1, 0),
                _ => hex(0, 0),
            };
            self.i += 1;
            Some(self.hex)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::geometry::{hex, Hex, Shape, Tri};

    #[test]
    fn hex_iter() {
        let t = Tri::new(hex(5, 5));
        assert_eq!(vec![hex(5, 5), hex(5, 4), hex(4, 5)], t.hex_iter().collect::<Vec<Hex>>());
    }

    #[test]
    fn chain_iter() {
        let t1 = Tri::new(hex(1, 1));
        let t2 = Tri::new(hex(10, 10));
        let joined = t1.hex_iter().chain(t2.hex_iter()).collect::<Vec<Hex>>();
        assert_eq!(6, joined.len());
    }
}