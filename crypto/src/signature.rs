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
mod tests {
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
    use primitive_types::{U256, U512};
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

use sha2::Sha256;

fn make_hash(source: &[u8]) -> U512 {
    let mut hasher = Sha256::new();
    hasher.update(source);
    U512::from(&hasher.finalize()[..])
}

// 署名ハッシュ作成
let z = FieldElement::new(make_hash(b"This is my sign"), n);

// 秘密鍵作成
let e = FieldElement::new(make_hash(b"This is my secret"), n);

// 乱数kを生成
use rand::Rng;
let mut rng = rand::thread_rng(); 
let i: i32 = rng.gen();
let k = FieldElement::new(U512::from(rng.gen::<i32>()), n);

let G = Point::new(gx, gy, a, b);
let r = (G * k).x;
let k_inv = FieldElement::new(k, n).pow(n-U512::from(2));
let s = (z + r * e) * k_inv;

let P = G * e;