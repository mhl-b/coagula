use super::{Direction, FHex, Hex};

pub trait Shaped {
    fn shape(&self) -> &impl Shape;
}

/// A geometrical shape of hexagonal tiles
pub trait Shape {

    /// an approximate center of the shape
    fn center(&self) -> Hex;

    /// given fractional hex position returns most relevant center hex
    fn nearest_center(&self, hex: FHex) -> Hex;

    /// number of tiles in the shape
    fn area(&self) -> usize;

    /// iterator of hex tiles in the shape with guaranteed order
    fn hex_iter(&self) -> impl Iterator<Item=Hex>;

    /// get hex by index using strong order
    fn hex_by_ind(&self, ind: usize) -> Option<Hex>;

    /// get index by hex using strong order
    fn ind_by_hex(&self, hex: Hex) -> Option<usize>;

    /// checks if shape contains hex
    fn contains(&self, hex: Hex) -> bool;

    /// create copy of shape at given hex as a center
    fn move_to(&self, hex: Hex) -> Self;

    /// iterator of all neighboring hexes, a counter of the shape
    fn all_neighbors(&self) -> impl Iterator<Item=Hex>;

    /// iterator of neighboring hexes in given direction
    fn direction_neighbors(&self, d: Direction) -> impl Iterator<Item=Hex>;
}

pub trait Overlap<S: Shape> {
    fn overlap_iter(&self, other: &S) -> impl Iterator<Item=Hex>;
    fn overlaps(&self, other: &S) -> bool;
}
