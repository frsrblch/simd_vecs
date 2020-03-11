use std::marker::PhantomData;
use std::ops::*;
use crate::ops::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Typed<VEC, UNIT> {
    pub vec: VEC,
    marker: PhantomData<UNIT>,
}

impl<'a, VEC1, VEC2, UNIT> AddAssign<&'a Typed<VEC2, UNIT>> for Typed<VEC1, UNIT>
where
    VEC1: AddAssign<&'a VEC2>,
{
    fn add_assign(&mut self, rhs: &'a Typed<VEC2, UNIT>) {
        self.vec += &rhs.vec;
    }
}

impl<'a, VEC1, VEC2, VEC3, UNIT1, UNIT2, UNIT3>
    AddAssign<(&'a Typed<VEC2, UNIT2>, &'a Typed<VEC3, UNIT3>)> for Typed<VEC1, UNIT1>
where
    VEC1: AddAssign<(&'a VEC2, &'a VEC3)>,
    UNIT2: Mul<UNIT3, Output = UNIT1>,
{
    fn add_assign(&mut self, rhs: (&'a Typed<VEC2, UNIT2>, &'a Typed<VEC3, UNIT3>)) {
        self.vec += (&rhs.0.vec, &rhs.1.vec);
    }
}

impl<'a, VEC1, VEC2, VEC3, UNIT1, UNIT2, UNIT3>
AddAssign<VMul<&'a Typed<VEC2, UNIT2>, &'a Typed<VEC3, UNIT3>>> for Typed<VEC1, UNIT1>
where
    VEC1: AddAssign<<&'a VEC2 as Mul<&'a VEC3>>::Output>,
    &'a VEC2: Mul<&'a VEC3>,
    UNIT2: Mul<UNIT3, Output = UNIT1>,
{
    fn add_assign(&mut self, rhs: VMul<&'a Typed<VEC2, UNIT2>, &'a Typed<VEC3, UNIT3>>) {
        self.vec += &rhs.a.vec * &rhs.b.vec;
    }
}

impl<'a, VEC1, UNIT1, VEC2, UNIT2> Mul<&'a Typed<VEC2, UNIT2>> for &'a Typed<VEC1, UNIT1> {
    type Output = VMul<&'a Typed<VEC1, UNIT1>, &'a Typed<VEC2, UNIT2>>;

    fn mul(self, rhs: &'a Typed<VEC2, UNIT2>) -> Self::Output {
        VMul {
            a: self,
            b: rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecs::*;
    use crate::units::*;

    #[test]
    fn add_assign_typed_vecs() {
        let mut position: Typed<Vec2<f32>, Meters> = Typed {
            vec: Vec2 {
                x: vec![1.0, 2.0].into(),
                y: vec![3.0, 5.0].into(),
            },
            marker: PhantomData,
        };

        let velocity: Typed<Vec2<f32>, MetersPerSecond> = Typed {
            vec: Vec2 {
                x: vec![2.0, 3.0].into(),
                y: vec![5.0, 7.0].into(),
            },
            marker: PhantomData,
        };

        let time: Typed<Vec1<f32>, Seconds> = Typed {
            vec: Vec1 {
                values: vec![1.0, 2.0].into(),
            },
            marker: PhantomData,
        };

        let expected: Typed<Vec2<f32>, Meters> = Typed {
            vec: Vec2 {
                x: vec![3.0, 8.0].into(),
                y: vec![8.0, 19.0].into(),
            },
            marker: PhantomData,
        };

        position += &velocity * &time;

        assert_eq!(expected, position);
    }
}