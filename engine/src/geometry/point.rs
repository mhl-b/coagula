use core::iter;

use super::{Direction, Hex, Shape};

pub struct Point {
    hex: Hex,
}

impl Shape for Point {

    fn center(&self) -> Hex {
        self.hex
    }

    fn area(&self) -> usize {
        1
    }

    fn hex_iter(&self) -> impl Iterator<Item = Hex> {
        iter::once(self.hex)
    }

    fn hex_by_ind(&self, ind: usize) -> Option<Hex> {
        if ind == 0 {
            Some(self.hex)
        } else {
            None
        }
    }

    fn ind_by_hex(&self, hex: Hex) -> Option<usize> {
        if self.hex == hex {
            Some(0)
        } else {
            None
        }
    }

    fn contains(&self, hex: Hex) -> bool {
        self.hex == hex
    }

    fn move_to(&self, hex: Hex) -> Self {
        Point { hex }
    }

    fn all_neighbors(&self) -> impl Iterator<Item = Hex> {
        self.hex.neighbors()
    }

    fn direction_neighbors(&self, d: Direction) -> impl Iterator<Item = Hex> {
        iter::once(self.hex.neighbor(d))
    }
}
