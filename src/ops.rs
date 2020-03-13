
pub trait VecOp<T1, T2> {
    type Output;
    fn function(b: T1, c: T2) -> Self::Output;
}

#[derive(Debug, Copy, Clone)]
pub struct VMul<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}

impl<'a, A, B, T1, T2> VecOp<T1, T2> for VMul<'a, A, B>
where
    T1: Copy + std::ops::Mul<T2>,
    T2: Copy,
{
    type Output = T1::Output;
    fn function(a: T1, b: T2) -> Self::Output {
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecs::*;
    use physics::*;

    #[test]
    fn typed_add_assign() {
        let mut position: Vec2<Length> = Vec2 {
            x: vec![0.0.into(), 1.0.into(), 2.0.into()].into(),
            y: vec![0.5.into(), 1.5.into(), 2.5.into()].into(),
        };

        let velocity: Vec2<Speed> = Vec2 {
            x: vec![0.5.into(), 1.5.into(), 2.5.into()].into(),
            y: vec![0.0.into(), 1.0.into(), 2.0.into()].into(),
        };

        let dt: Vec1<Time> = Vec1 {
            values: vec![2.0.into(), 2.0.into(), 2.0.into()],
        };

        let expected = Vec2 {
            x: vec![1.0.into(), 4.0.into(), 7.0.into()].into(),
            y: vec![0.5.into(), 3.5.into(), 6.5.into()].into(),
        };

        position += &velocity * &dt;

        assert_eq!(expected, position);
    }
}