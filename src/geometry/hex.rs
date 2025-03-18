use std::ops::{Add, Mul, Sub};

// https://www.redblobgames.com/grids/hexagons/

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IHex(pub i32, pub i32);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FHex(pub f32, pub f32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HexDir {
    BotRight = 0,
    Right,
    TopRight,
    TopLeft,
    Left,
    BotLeft,
}
use HexDir::*;

pub const IHEX_0: IHex = IHex(0, 0);
pub const FHEX_0: FHex = FHex(0.0, 0.0);

pub const ALL_DIR: [HexDir; 6] = [BotRight, Right, TopRight, TopLeft, Left, BotLeft];

impl HexDir {
    pub fn to_ind(self) -> usize {
        match self {
            BotRight => 0,
            Right => 1,
            TopRight => 2,
            TopLeft => 3,
            Left => 4,
            BotLeft => 5,
        }
    }
}

impl From<usize> for HexDir {
    fn from(value: usize) -> Self {
        match value {
            0 => BotRight,
            1 => Right,
            2 => TopRight,
            3 => TopLeft,
            4 => Left,
            _ => BotLeft,
        }
    }
}

impl From<HexDir> for IHex {
    fn from(value: HexDir) -> Self {
        match value {
            BotRight => IHex(0, 1),
            Right => IHex(1, 0),
            TopRight => IHex(1, -1),
            TopLeft => IHex(0, -1),
            Left => IHex(-1, 0),
            BotLeft => IHex(-1, 1),
        }
    }
}

impl IHex {
    pub fn q(&self) -> i32 {
        self.0
    }

    pub fn r(&self) -> i32 {
        self.1
    }

    pub fn s(&self) -> i32 {
        -self.0 - self.1
    }

    pub fn length(self) -> i32 {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }

    pub fn distance(self, other: IHex) -> i32 {
        (self - other).length()
    }

    pub fn lerp(self, other: IHex, t: f32) -> FHex {
        FHex(
            lerp(self.q() as f32, other.q() as f32, t),
            lerp(self.r() as f32, other.r() as f32, t),
        )
    }

    pub fn line(&self, other: IHex, out: &mut [IHex]) -> usize {
        let dist = self.distance(other) as usize;
        assert!(dist < out.len());
        let step = 1.0 / (dist.max(1) as f32);
        (0..=dist).for_each(|i| {
            unsafe { *out.get_unchecked_mut(i) = self.lerp(other, step * (i as f32)).round() };
        });
        dist
    }

    pub fn line_vec(self, other: IHex) -> Vec<IHex> {
        let dist = self.distance(other);
        let step = 1.0 / (dist.max(1) as f32);
        (0..=dist)
            .map(|i| self.lerp(other, step * (i as f32)).round())
            .collect()
    }

    pub fn neighbors(self) -> [IHex; 6] {
        let mut out = [IHEX_0; 6];
        for i in 0..6 {
            out[i] = self + IHex::from(ALL_DIR[i]);
        }
        out
    }
}

impl Add for IHex {
    type Output = IHex;

    fn add(self, rhs: Self) -> Self::Output {
        IHex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<HexDir> for IHex {
    type Output = IHex;

    fn add(self, rhs: HexDir) -> Self::Output {
        self + IHex::from(rhs)
    }
}

impl Sub for IHex {
    type Output = IHex;

    fn sub(self, rhs: Self) -> Self::Output {
        IHex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for IHex {
    type Output = IHex;

    fn mul(self, rhs: i32) -> Self::Output {
        IHex(self.0 * rhs, self.1 * rhs)
    }
}

impl FHex {
    pub fn q(&self) -> f32 {
        self.0
    }

    pub fn r(&self) -> f32 {
        self.1
    }

    pub fn s(&self) -> f32 {
        -self.0 - self.1
    }

    pub fn round(&self) -> IHex {
        let mut q = self.q().round();
        let mut r = self.r().round();
        let s = self.s().round();
        let dq = q - self.q();
        let dr = r - self.r();
        let ds = s - self.s();
        if dq > dr && dq > ds {
            q = -r - s;
        } else if dr > ds {
            r = -q - s;
        }
        IHex(q as i32, r as i32)
    }
}

impl Add for FHex {
    type Output = FHex;

    fn add(self, rhs: Self) -> Self::Output {
        FHex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for FHex {
    type Output = FHex;

    fn sub(self, rhs: Self) -> Self::Output {
        FHex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f32> for FHex {
    type Output = FHex;

    fn mul(self, rhs: f32) -> Self::Output {
        FHex(self.0 * rhs, self.1 * rhs)
    }
}

impl From<IHex> for FHex {
    fn from(h: IHex) -> Self {
        FHex(h.0 as f32, h.1 as f32)
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1. - t) + b * t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_add() {
        assert_eq!(IHex(5, 3), IHex(2, 1) + IHex(3, 2))
    }

    #[test]
    fn hex_sub() {
        assert_eq!(IHex(5, 3), IHex(-1, -2) - IHex(-6, -5))
    }

    #[test]
    fn hex_mul() {
        assert_eq!(IHex(20, 30), IHex(2, 3) * 10);
    }

    #[test]
    fn fhex_add() {
        assert_eq!(FHex(2.1 + 3.1, 1.3 + 2.3), FHex(2.1, 1.3) + FHex(3.1, 2.3))
    }

    #[test]
    fn fhex_mul() {
        assert_eq!(FHex(2.1 * 5., 1.3 * 5.), FHex(2.1, 1.3) * 5.)
    }

    #[test]
    fn hex_len() {
        assert_eq!(5, IHex(2, 3).length())
    }

    #[test]
    fn hex_dist() {
        assert_eq!(5, IHex(-3, 1).distance(IHex(2, -1)));
    }

    #[test]
    fn hex_line() {
        let want = [IHex(-2, -1), IHex(-2, 0), IHex(-1, 0), IHex(-1, 1)];
        let mut got = [IHex(0, 0); 4];
        IHex(-2, -1).line(IHex(-1, 1), &mut got);
        assert_eq!(&want, &got);
    }
}
