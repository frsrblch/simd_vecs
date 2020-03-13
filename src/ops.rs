#[derive(Debug, Copy, Clone)]
pub struct VMul<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}

#[derive(Debug, Copy, Clone)]
pub struct VDiv<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
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