use super::*;

pub trait Shaped {
    fn shape(&self) -> &impl Shape;
}

/// A geometrical shape of hexagonal tiles
pub trait Shape {
    /// an approximate center of the shape
    fn center(&self) -> IHex;

    /// given fractional hex position returns most relevant center hex
    fn nearest_center(&self, hex: FHex) -> IHex;

    /// number of tiles in the shape
    fn area(&self) -> usize;

    /// iterator of hex tiles in the shape with guaranteed order
    fn hex_iter(&self) -> impl Iterator<Item = IHex>;

    /// get hex by index using strong order
    fn hex_by_ind(&self, ind: usize) -> Option<IHex>;

    /// get index by hex using strong order
    fn ind_by_hex(&self, hex: IHex) -> Option<usize>;

    /// checks if shape contains hex
    fn contains(&self, hex: IHex) -> bool;

    /// create copy of shape at given hex as a center
    fn move_to(&self, hex: IHex) -> Self;

    /// iterator of all neighboring hexes, outer perimiter of the shape
    fn all_neighbors(&self) -> impl Iterator<Item = IHex>;

    /// iterator of neighboring hexes in given direction
    fn direction_neighbors(&self, d: HexDir) -> impl Iterator<Item = IHex>;
}

pub trait Overlap<S: Shape> {
    fn overlap_iter(&self, other: &S) -> impl Iterator<Item = IHex>;
    fn overlaps(&self, other: &S) -> bool;
}
