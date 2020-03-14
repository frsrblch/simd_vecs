use crate::ops::*;
use num_traits::Float;
use std::ops::*;

pub use vec1::Vec1;
pub use vec2::Vec2;

mod vec1;
mod vec2;

#[cfg(test)]
mod tests {
    use super::*;
    use physics::{Length, Speed, Time};

    #[test]
    fn add_assign_vec1_to_vec1() {
        let mut v1: Vec1<Length> = Vec1 {
            values: vec![1.0.into(), 2.0.into()],
        };

        let v2: Vec1<Length> = Vec1 {
            values: vec![7.0.into(), 11.0.into()],
        };

        let expected = Vec1 {
            values: vec![8.0.into(), 13.0.into()],
        };

        v1 += &v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn add_assign_vec2_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![1.0, 2.0].into(),
            y: vec![3.0, 5.0].into(),
        };

        let v2 = Vec2 {
            x: vec![7.0, 11.0].into(),
            y: vec![13.0, 17.0].into(),
        };

        let expected = Vec2 {
            x: vec![8.0, 13.0].into(),
            y: vec![16.0, 22.0].into(),
        };

        v1 += &v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn add_assign_magnitude() {
        let mut v1 = Vec1 {
            values: vec![1.0, 2.0],
        };

        let v2 = Vec2 {
            x: vec![3.0, 0.0].into(),
            y: vec![4.0, 1.0].into(),
        };

        let expected = Vec1 {
            values: vec![6.0, 3.0],
        };

        v1.zip_to_vec2(&v2, |v, x, y| *v += (x * x + y * y).sqrt());

        assert_eq!(expected, v1);
    }

    #[test]
    fn sub_assign_vec2_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![1.0, 2.0].into(),
            y: vec![3.0, 5.0].into(),
        };

        let v2 = Vec2 {
            x: vec![7.0, 11.0].into(),
            y: vec![13.0, 17.0].into(),
        };

        let expected = Vec2 {
            x: vec![-6.0, -9.0].into(),
            y: vec![-10.0, -12.0].into(),
        };

        v1 -= &v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn mul_assign_vec1_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![1.0f32, 2.0].into(),
            y: vec![3.0, 5.0].into(),
        };

        let v2 = Vec1::from(vec![2.0, 3.0]);

        let expected = Vec2 {
            x: vec![2.0, 6.0].into(),
            y: vec![6.0, 15.0].into(),
        };

        v1 *= &v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn mul_assign_value_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![1.0, 3.0].into(),
            y: vec![3.0, 7.5].into(),
        };

        let v2 = 2.0;

        let expected = Vec2 {
            x: vec![2.0, 6.0].into(),
            y: vec![6.0, 15.0].into(),
        };

        v1 *= v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn div_assign_vec1_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![2.0, 6.0].into(),
            y: vec![6.0, 15.0].into(),
        };

        let v2 = Vec1::from(vec![2.0, 3.0]);

        let expected = Vec2 {
            x: vec![1.0, 2.0].into(),
            y: vec![3.0, 5.0].into(),
        };

        v1 /= &v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn div_assign_value_to_vec2() {
        let mut v1 = Vec2 {
            x: vec![2.0, 6.0].into(),
            y: vec![6.0, 15.0].into(),
        };

        let v2 = 2.0;

        let expected = Vec2 {
            x: vec![1.0, 3.0].into(),
            y: vec![3.0, 7.5].into(),
        };

        v1 /= v2;

        assert_eq!(expected, v1);
    }

    #[test]
    fn add_assign_mul_vec1() {
        let mut position: Vec1<Length> = Vec1 {
            values: vec![0.0.into(), 1.0.into(), 2.0.into()]
        };

        let speed: Vec1<Speed> = Vec1 {
            values: vec![2.0.into(), 3.0.into(), 5.0.into()],
        };

        let time: Vec1<Time> = Vec1 {
            values: vec![1.0.into(), 2.0.into(), 4.0.into()]
        };

        position += VMul::new(&speed, &time);

        let expected = Vec1 {
            values: vec![2.0.into(), 7.0.into(), 22.0.into()]
        };

        assert_eq!(expected, position);
    }

    #[test]
    fn add_assign_mul_vec1_value() {
        let mut position: Vec1<Length> = Vec1 {
            values: vec![0.0.into(), 1.0.into(), 2.0.into()]
        };

        let speed: Vec1<Speed> = Vec1 {
            values: vec![2.0.into(), 3.0.into(), 5.0.into()],
        };

        let time = Time::in_seconds(2.0);

        position += VMul::new(&speed, &time);

        let expected = Vec1 {
            values: vec![4.0.into(), 7.0.into(), 12.0.into()]
        };

        assert_eq!(expected, position);
    }

    #[test]
    fn sub_assign_mul_vec1_value() {
        let mut position: Vec1<Length> = Vec1 {
            values: vec![0.0.into(), 1.0.into(), 2.0.into()]
        };

        let speed: Vec1<Speed> = Vec1 {
            values: vec![2.0.into(), 3.0.into(), 5.0.into()],
        };

        let time = Time::in_seconds(2.0);

        position -= VMul::new(&speed, &time);

        let expected = Vec1 {
            values: vec![(-4.0).into(), (-5.0).into(), (-8.0).into()]
        };

        assert_eq!(expected, position);
    }
}
