// Elliptic Curve: y^2 = x^3 + a*x + b
#[derive(Clone, Debug)]
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
mod tests {
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
mod tests {
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
