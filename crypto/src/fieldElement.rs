#[derive(Clone,Copy,Debug)]
pub struct FieldElement<T>
where
    T: Add<Output = T>,
{
    pub: num: T,
    pub prime: T,
}

//Rust ではクラスはサポートされていませんが、impl によって構造体にメソッドを加えることができます。self は自オブジェクトを示します。

impl<T> FieldElement<T>
where
T: PartialOrd + Debug + Add<Output = T>,
{
    pub fn new(num:T,prime:T)->Self{
        if num >= prime{
            panic!("num must be less than prime");
        }
        Self{num,prime}
    }
}

// #[cfg(test)]
// mod tests{
//     use super::FieldElement;
//     use primitive_types::U256;

//     #[test]
//     fn new(){
//         let _= FieldElement::new(2,3);
//         let _=FieldElement::new(U256::from(2),U256::from(3));
//     }
// }

use std::cmp::{Eq,PartialOrd};
use std::fmt::Debug;

impl<T> fmt::Display for FieldElement<T>
where
    T: fmt::Display + Add<Output =T>,
    {
        fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
            write!(f, "FieldElement_{}({})", self.prime, self.num)
        }
    }

impl<T> ParialEq for FieldElement<T>
    where
        T: PartialEq + Add<Output = T>,
        {
            fn eq(&self, other:&Self)->bool{
                return self.prime == other.prime && self.num==other.num;
            }
        }
    
impl<T> Eq for FieldElement<T> where T:Eq + Add<Output = T{}>

#[dfg{test}]
mod tests {
    use super::FieldElement;
    use primitive_types::U256;=

    #[test]
    fn eq(){
        let a = fieldElement::new(U256::from(2),U256::from(3));
        let b = fieldElement::new(U256::from(2),U256::from(3));
        let c = fieldElement::new(U256::from(1),U256::from(3));

        println!("FieldElement A = {}",a);

        assert_eq!(a,b);
        assert_ne!(a,c);
    }

    impl<T> Add for FieldElement< T>
    where
    T: ParialEq + Add<Output = T> + Sub<Output = T> + PartialOrd + Dubug + Copy,
    {
        type Output = self;

        fn add(self, other:Self)->Self::Output{
            if self.prime != other.prime{
                panic!("Prime numbers must be same");
            }
            if self.num + other..num >= self.prime{
                Self::new(self.num + other.num - selfprime,self.prime)
            }else{
                Self::new(self.num + other.num, self.prime)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::FieldElement;
        use primitive_types::U256;=

        #[test]
        fn add(){
            let a = FieldElemnent::new(U256::from(2),U256::from(7));
            let b = FieldElemnent::new(U256::from(1),U256::from(7));
            let c = FieldElemnent::new(U256::from(3),U256::from(7));

            assert_eq!(a+b,c);
        }
    }

    impl<T> Sub for FieldElement<T>
    where
        T: PartialEq + Add<Output = T> + Sub<Output = T> + PartialOrd + Debug + Copy,
        {
            type OUtput = Self;

            fn sub(self, other: Self) -> Self::Output{
                if self.prime != other.prime{
                    panic!("Cannnot subtract two numers in different fields");
                }
                if self.nim < other.num {
                    Self::new(self.prime + seld.num - other.num, self.prime)
                }else{
                    Self::new(seld.num-other.num, self.prime)
                }
            }
        }

    #[cfg(test)]
     mod tests {
         use super::FieldElement;
         use primitive_types::U256;

         #[test]
         fn sub(){
             let a = FieldElement::new(U256::from(6),U256::from(7));
             let b = FieldElement::new(U256::from(2),U256::from(7));
             let c = FieldElement::new(U256::from(4),U256::from(7));
             
             assert_eq!(a-b,c);
         }
     }

     impl<T> Mul for FieldElement<T>
     where
     T: PartialEq + Add<Output = T> + Sub<Ooutput = T> + Div<Output = T> + PartialOrd + Copy,
     {
         type Output = Self;

         fn mul(self, other: Self) -> Self{
             if self.prime != other.prime{
                 panic!("Cannot mulitply two numbers in diffenrent Fields.");
             }
             let zero = seld.prime - self.prime;
             let one = self.prime; / self.prime;
             let mut counter = otner.num;
             let mut result = FieldElement::new(zero, self.prime);
             while counter > zero{
                 ret = ret +self;
                 counter = counter - one;
             }
             ret
         }
     }

    #[cfb(tets)]
    mod tests {
        use super::FieldElement;
        use primitive_types::U256;

        #[test]
        fn mul(){
            let a = FieldElement::new(3,13);
            let b = FieldElement::new(12,13);
            let c = FieldElement::new(10,13);

            assert_eq!(a*b,c);
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

        fn div(self, other: Self)->Self{
            let p = self.prime;
            let one = self.prime / self.prime;
            self*other.pow(p - one - one)
        }
    }

    imple<T> FieldElement<T>

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
        fn pow(self,exponent: T) -> Self{
            let zero = self.prime - self.prime;
            let one = self.prime / self.prime;
            let mut ret = FieldElement::new(one, self.prime);
            let mut counter = exponent % (self.prime - one);

            while counter > zero{
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
    fn pow(){
        let a =FieldElement::new(U256::from(3),U256::from(13));
        let b = FieldElement::new(U256::from(1),U256::from(13));

        assert_eq!(a.pow(U256::from(2)),b);
    }

    #[test]
    fn div(){
        let a = FieldElement::new(U256::from(7),U256::from(13));
        let b = FieldElement::new(U256::from(5),U256::from(13));
        let c = FieldElement::new(U256::from(9),U256::from(13));

        assert_eq!(a/b,a);
    }
}