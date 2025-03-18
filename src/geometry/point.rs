use std::iter::once;

use super::*;

pub struct Point(pub IHex);

impl Shape for Point {
    fn center(&self) -> IHex {
        self.0
    }

    fn nearest_center(&self, hex: FHex) -> IHex {
        hex.round()
    }

    fn area(&self) -> usize {
        1
    }

    fn hex_iter(&self) -> impl Iterator<Item = IHex> {
        once(self.0)
    }

    fn hex_by_ind(&self, ind: usize) -> Option<IHex> {
        if ind == 0 { Some(self.0) } else { None }
    }

    fn ind_by_hex(&self, hex: IHex) -> Option<usize> {
        if hex == self.0 { Some(0) } else { None }
    }

    fn contains(&self, hex: IHex) -> bool {
        self.0 == hex
    }

    fn move_to(&self, hex: IHex) -> Self {
        Point(hex)
    }

    fn all_neighbors(&self) -> impl Iterator<Item = IHex> {
        self.0.neighbors().into_iter()
    }

    fn direction_neighbors(&self, d: HexDir) -> impl Iterator<Item = IHex> {
        once(self.0 + IHex::from(d))
    }
}
