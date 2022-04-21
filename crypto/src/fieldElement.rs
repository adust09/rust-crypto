use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]

pub struct FieldElement<T>
where
    T: Add<Output = T>,
{
    pub num: T,
    pub prime: T,
}

impl<T> FieldElement<T>
where
    T: PartialOrd + Debug + Add<Output = T>,
{
    pub fn new(num: T, prime: T) -> Self {
        if num >= prime {
            panic!("Num {:?} not in field range 0 to {:?}", num, prime)
        }
        Self { num, prime }
    }
}

#[cfg(test)]
mod fieldElementtests {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn new() {
        let _ = FieldElement::new(2, 3);
        let _ = FieldElement::new(U256::from(2), U256::from(3));
    }
}

use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::fmt::Debug;

impl<T> fmt::Display for FieldElement<T>
where
    T: fmt::Display + Add<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl<T> PartialEq for FieldElement<T>
where
    T: PartialEq + Add<Output = T>,
{
    fn eq(&self, other: &Self) -> bool {
        return self.prime == other.prime && self.num == other.num;
    }
}

impl<T> Eq for FieldElement<T> where T: Eq + Add<Output = T> {}

#[cfg(test)]
mod fieldElementPointTest {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn eq() {
        let a = FieldElement::new(U256::from(2), U256::from(3));
        let b = FieldElement::new(U256::from(2), U256::from(3));
        let c = FieldElement::new(U256::from(1), U256::from(3));

        println!("FieldElement A = {}", a);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}

impl<T> Add for FieldElement<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + PartialOrd + Debug + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Prime number should be same")
        }
        if self.num + other.num >= self.prime {
            Self::new(self.num + other.num - self.prime, self.prime)
        } else {
            Self::new(self.num + other.num, self.prime)
        }
    }
}

#[cfg(test)]
mod fieldElementAddTest {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn add() {
        let a = FieldElement::new(U256::from(2), U256::from(7));
        let b = FieldElement::new(U256::from(1), U256::from(7));
        let c = FieldElement::new(U256::from(3), U256::from(7));

        assert_eq!(a + b, c);
    }
}

impl<T> Sub for FieldElement<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + PartialOrd + Debug + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields.");
        }
        if self.num < other.num {
            Self::new(self.prime + self.num - other.num, self.prime)
        } else {
            Self::new(self.num - other.num, self.prime)
        }
    }
}

#[cfg(test)]
mod fieldElementSubTest {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn sub() {
        let a = FieldElement::new(U256::from(6), U256::from(7));
        let b = FieldElement::new(U256::from(4), U256::from(7));
        let c = FieldElement::new(U256::from(2), U256::from(7));

        assert_eq!(a - b, c);
    }
}

impl<T> Mul for FieldElement<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + Div<Output = T> + PartialOrd + Debug + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields.");
        }
        let zero = self.prime - self.prime;
        let one = self.prime / self.prime;
        let mut counter = other.num;
        let mut ret = FieldElement::new(zero, self.prime);
        while counter > zero {
            ret = ret + self;
            counter = counter - one;
        }
        ret
    }
}

#[cfg(test)]
mod fieldElementMulTest {
    use super::FieldElement;

    #[test]
    fn mul() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);

        assert_eq!(a * b, c);
    }
}

impl<T> Div for FieldElement<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Debug
        + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let p = self.prime;
        let one = self.prime / self.prime;
        self * other.pow(p - one - one)
    }
}

impl<T> FieldElement<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Debug
        + Copy,
{
    fn pow(self, exponent: T) -> Self {
        let zero = self.prime - self.prime;
        let one = self.prime / self.prime;
        let mut ret = FieldElement::new(one, self.prime);
        let mut counter = exponent % (self.prime - one);

        while counter > zero {
            ret = ret * self;
            counter = counter - one;
        }
        ret
    }
}

#[cfg(test)]
mod fieldElementDivTest {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn pow() {
        let a = FieldElement::new(U256::from(3), U256::from(13));
        let b = FieldElement::new(U256::from(1), U256::from(13));

        assert_eq!(a.pow(U256::from(3)), b);
    }

    #[test]
    fn div() {
        let a = FieldElement::new(U256::from(7), U256::from(19));
        let b = FieldElement::new(U256::from(5), U256::from(19));
        let c = FieldElement::new(U256::from(9), U256::from(19));

        assert_eq!(a / b, c);
    }
}

// Elliptic Curve: y^2 = x^3 + a*x + b
#[derive(Clone, Debug, PartialEq)]
pub enum Point<T> {
    Coordinate { x: T, y: T, a: T, b: T },
    Infinity,
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Point::Coordinate { x, y, a, b } => {
                write!(f, "Point({}, {})_{}_{}", x, y, a, b)
            }
            &Point::Infinity => {
                write!(f, "Point(infinity)")
            }
        }
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Mul<Output = T> + PartialEq + Copy,
{
    pub fn new(x: T, y: T, a: T, b: T) -> Self {
        if y * y != x * x * x + a * x + b {
            panic!("This is invalid number.");
        }
        Self::Coordinate { x, y, a, b }
    }
}

#[cfg(test)]
mod curvePointTests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn new() {
        let _ = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));
    }

    #[test]
    fn eq() {
        let a = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));
        let b = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));

        assert!(a == b);
    }
}

impl<T> Add for Point<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use Point::*;

        match (self, other) {
            (
                Coordinate {
                    x: x0,
                    y: y0,
                    a: a0,
                    b: b0,
                },
                Coordinate {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
            ) => {
                if a0 != a1 || b0 != b1 {
                    panic!("Points are not on the same curve.")
                }
                if x0 == x1 {
                    if y0 == y1 - y1 {
                        return Infinity;
                    }
                    let one = a0 / a0;
                    let two = one + one;
                    let three = one + one + one;
                    let s = (three * x0 * x0 + a0) / (two * y0);
                    let x2 = s * s - two * x0;
                    return Coordinate {
                        x: x2,
                        y: s * (x0 - x2) - y0,
                        a: a0,
                        b: b0,
                    };
                }
                let s = (y1 - y0) / (x1 - x0);

                let x2 = s * s - x1 - x0;
                let y2 = s * (x0 - x2) - y0;
                return Coordinate {
                    x: x2,
                    y: y2,
                    a: a0,
                    b: b0,
                };
            }
            (Coordinate { x, y, a, b }, Infinity) => Coordinate { x, y, a, b },
            (Infinity, Coordinate { x, y, a, b }) => Coordinate { x, y, a, b },
            (Infinity, Infinity) => Infinity,
        }
    }
}

#[cfg(test)]
mod curveTests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn point_on_elliptic_curve() {
        let a = FieldElement::new(U256::from(0), U256::from(223));
        let b = FieldElement::new(U256::from(7), U256::from(223));
        let x = FieldElement::new(U256::from(192), U256::from(223));
        let y = FieldElement::new(U256::from(105), U256::from(223));

        assert_eq!(y * y, x * x * x + a * x + b);
    }

    #[test]
    fn add_points() {
        let a = FieldElement::new(U256::from(0), U256::from(223));
        let b = FieldElement::new(U256::from(7), U256::from(223));
        let x0 = FieldElement::new(U256::from(192), U256::from(223));
        let y0 = FieldElement::new(U256::from(105), U256::from(223));
        let x1 = FieldElement::new(U256::from(17), U256::from(223));
        let y1 = FieldElement::new(U256::from(56), U256::from(223));
        let x2 = FieldElement::new(U256::from(170), U256::from(223));
        let y2 = FieldElement::new(U256::from(142), U256::from(223));

        let p0 = Point::new(x0, y0, a, b);
        let p1 = Point::new(x1, y1, a, b);
        let p2 = Point::new(x2, y2, a, b);

        assert_ne!(p0, p1);
        assert_eq!(p0 + p1, p2);
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + PartialOrd + Copy,
    U: Sub<Output = U> + Div<Output = U> + Mul<Output = U> + PartialOrd + Copy,
{
    type Output = Point<T>;

    fn mul(self, other: U) -> Self::Output {
        let zero = other - other;
        let one = other / other;
        let mut counter = other;
        let mut ret = Self::Infinity;
        while counter > zero {
            ret = ret + self;
            counter = counter - one;
        }
        ret
    }
}

#[cfg(test)]
mod curvePoinTests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn mul() {
        let p0 = Point::new(2, 5, 5, 7);
        let p1 = Point::new(2, -5, 5, 7);

        assert_ne!(p0, p1);
        assert_eq!(p0 * 3, p1);
        assert_eq!(p0 * U256::from(3), p1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U512;
    #[test]
    fn on_the_curve() {
        let p = U512::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap();
        let x = U512::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let y = U512::from_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .unwrap();
        let n = U512::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap();

        let a = FieldElement::new(U512::from(0), p);
        let b = FieldElement::new(U512::from(7), p);
        let gx = FieldElement::new(x, p);
        let gy = FieldElement::new(y, p);

        let _ = Point::new(gx, gy, a, b);
    }
}

// use sha2::Sha256;

// fn make_hash(source: &[u8]) -> U512 {
//     let mut hasher = Sha256::new();
//     hasher.update(source);
//     U512::from(&hasher.finalize()[..])
// }

// fn a() {
//     // 署名ハッシュ作成
//     let z = FieldElement::new(make_hash(b"This is my sign"), n);

//     // 秘密鍵作成
//     let e = FieldElement::new(make_hash(b"This is my secret"), n);

//     // 乱数kを生成
//     use rand::Rng;
//     let mut rng = rand::thread_rng();
//     let i: i32 = rng.gen();
//     let k = FieldElement::new(U512::from(rng.gen::<i32>()), n);

//     let G = Point::new(gx, gy, a, b);
//     let r = (G * k).x;
//     let k_inv = FieldElement::new(k, n).pow(n - U512::from(2));
//     let s = (z + r * e) * k_inv;

//     let P = G * e;
// }
