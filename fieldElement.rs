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
    T:fmt::Display + Add<Output =T>,
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