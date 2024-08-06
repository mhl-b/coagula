use core::ops::{Add, AddAssign, Mul, Sub};

use libm::roundf;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hex {
    q: i32,
    r: i32,
}

pub type Direction = usize;

pub const HEX0: Hex = hex(0,0);

// counter-clock q-axis
pub const HEX_DIRECTIONS: [Hex; 6] = [
    hex(1, 0),
    hex(1, -1),
    hex(0, -1),
    hex(1, 0),
    hex(-1, 1),
    hex(0, 1),
];

pub const fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

pub const fn hexf(q: f32, r: f32) -> Hex {
    Hex {
        q: q as i32,
        r: r as i32,
    }
}

impl Hex {
    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn length(&self) -> i32 {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }

    pub fn distance(self, other: Hex) -> i32 {
        (self - other).length()
    }

    pub fn direction(d: Direction) -> Hex {
        assert!((0..6).contains(&d));
        HEX_DIRECTIONS[d]
    }

    pub fn neighbor(self, d: Direction) -> Hex {
        self + Hex::direction(d)
    }

    pub fn neighbors(self) -> impl Iterator<Item = Hex> {
        (0..6).map(move |d| self.neighbor(d))
    }

    pub fn lerp(self, other: Hex, t: f32) -> FHex {
        FHex {
            q: lerp(self.q() as f32, other.q() as f32, t),
            r: lerp(self.r() as f32, other.r() as f32, t),
        }
    }

    pub fn line(&self, other: Hex, out: &mut [Hex]) {
        let dist = self.distance(other);
        assert!((dist as usize) < out.len());
        let step = 1.0 / (dist.max(1) as f32);
        (0..=dist).for_each(|i| {
            out[i as usize] = self.lerp(other, step * (i as f32)).round();
        })
    }

    pub fn hash(&self) -> u64 {
        ((self.q as u64) << 32) | self.r as u64
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, rhs: Self) -> Self::Output {
        hex(self.q + rhs.q, self.r + rhs.r)
    }
}

impl AddAssign for Hex {
    fn add_assign(&mut self, rhs: Self) {
        self.q = self.q + rhs.r;
        self.r = self.r + rhs.r;
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, rhs: Self) -> Self::Output {
        hex(self.q - rhs.q, self.r - rhs.r)
    }
}

impl Mul<i32> for Hex {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Self::Output {
        hex(self.q * rhs, self.r * rhs)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FHex {
    q: f32,
    r: f32,
}

impl FHex {
    pub fn ax(q: f32, r: f32) -> FHex {
        FHex { q, r }
    }

    pub fn q(&self) -> f32 {
        self.q
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn s(&self) -> f32 {
        -self.q - self.r
    }

    pub fn round(&self) -> Hex {
        let mut q = roundf(self.q());
        let mut r = roundf(self.r());
        let s = roundf(self.s());
        let dq = q - self.q();
        let dr = r - self.r();
        let ds = s - self.s();
        if dq > dr && dq > ds {
            q = -r - s;
        } else if dr > ds {
            r = -q - s;
        }
        hexf(q, r)
    }
}

impl Add for FHex {
    type Output = FHex;

    fn add(self, rhs: Self) -> Self::Output {
        FHex {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub for FHex {
    type Output = FHex;

    fn sub(self, rhs: Self) -> Self::Output {
        FHex {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Mul<f32> for FHex {
    type Output = FHex;

    fn mul(self, rhs: f32) -> Self::Output {
        FHex {
            q: self.q * rhs,
            r: self.r * rhs,
        }
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1. - t) + b * t
}

#[cfg(test)]
mod test {
    use crate::geometry::{hex, FHex, Hex};

    #[test]
    fn hex_add() {
        assert_eq!(hex(5, 3), hex(2, 1) + hex(3, 2))
    }

    #[test]
    fn hex_sub() {
        assert_eq!(hex(5, 3), hex(-1, -2) - hex(-6, -5))
    }

    #[test]
    fn hex_mul() {
        assert_eq!(hex(20, 30), hex(2, 3) * 10);
    }

    #[test]
    fn fhex_add() {
        assert_eq!(
            FHex::ax(2.1 + 3.1, 1.3 + 2.3),
            FHex::ax(2.1, 1.3) + FHex::ax(3.1, 2.3)
        )
    }

    #[test]
    fn fhex_mul() {
        assert_eq!(FHex::ax(2.1 * 5., 1.3 * 5.), FHex::ax(2.1, 1.3,) * 5.)
    }

    #[test]
    fn hex_len() {
        assert_eq!(5, hex(2, 3).length())
    }

    #[test]
    fn hex_dist() {
        assert_eq!(5, hex(-3, 1).distance(hex(2, -1)));
    }

    #[test]
    fn hex_line() {
        let want = [hex(-2, -1), hex(-2, 0), hex(-1, 0), hex(-1, 1)];
        let mut got = [hex(0, 0); 4];
        hex(-2, -1).line(hex(-1, 1), &mut got);
        assert_eq!(&want, &got);
    }
}
