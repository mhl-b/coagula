use hex::{HEX0, HEX_DIRECTIONS};

use super::{Direction, hex, Hex, Shape};

/// special case pointy top triangle with only 3 hexes
///   (1,0)
///(0,1) (1,1)
pub struct Point3 {
    // right -> top -> left
    v: [Hex; 3],
}

impl Point3 {
    pub fn new(h: Hex) -> Self {
        Self {
            v: [h, h - hex(0, 1), h - hex(1, 0)],
        }
    }
}

impl Shape for Point3 {
    fn center(&self) -> Hex {
        self.v[0]
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
        Point3::new(hex)
    }

    fn all_neighbors(&self) -> impl Iterator<Item=Hex> {
        Point3AllNeighborsIter::new(self.v[0])
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

struct Point3AllNeighborsIter {
    i: usize,
    hex: Hex,
}

impl Point3AllNeighborsIter {
    fn new(hex: Hex) -> Self {
        Self { i: 0, hex }
    }
}

impl Iterator for Point3AllNeighborsIter {
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
