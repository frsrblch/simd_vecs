use std::ops::*;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub struct Typed<VEC, UNIT> {
    pub vec: VEC,
    marker: PhantomData<UNIT>,
}

impl<'a, VEC1: AddAssign<&'a VEC2>, VEC2, UNIT> AddAssign<&'a Typed<VEC2, UNIT>> for Typed<VEC1, UNIT> {
    fn add_assign(&mut self, rhs: &'a Typed<VEC2, UNIT>) {
        self.vec += &rhs.vec;
    }
}