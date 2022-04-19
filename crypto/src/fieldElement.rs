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
mod tests {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn new() {
        let _ = FieldElement::new(2, 3);
        let _ = FieldElement::new(U256::from(2), U256::from(3));
    }
}

use std::cmp::{Eq, PartialEq};
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
mod tests {
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
mod tests {
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
mod tests {
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
mod tests {
    use super::FieldElement;
    use primitive_types::U256;

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
mod tests {
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
