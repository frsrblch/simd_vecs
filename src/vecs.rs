use std::iter::{once, repeat};
use std::ops::*;
use num_traits::Float;

pub trait ZipAndThen<I, T, F> {
    fn zip_and_then(&mut self, rhs: I, f: F);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec1<T> {
    pub values: Vec<T>,
}

impl<T> Default for Vec1<T> {
    fn default() -> Self {
        Vec1 {
            values: vec![],
        }
    }
}

impl<T> Vec1<T> {
    pub fn new() -> Self {
        Vec1 { values: vec![] }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.values.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn as_slice(&self) -> &[T] {
        self.values.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.values.as_mut_slice()
    }
}

impl<T> From<Vec<T>> for Vec1<T> {
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<'a, T: Copy, F: Fn(&mut T, T)> ZipAndThen<&'a Vec1<T>, T, F> for Vec1<T> {
    fn zip_and_then(&mut self, rhs: &Vec1<T>, f: F) {
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| f(lhs, *rhs))
    }
}

impl<'a, T: Copy, F: Fn(&mut T, T, T)> ZipAndThen<&'a Vec2<T>, T, F> for Vec1<T> {
    fn zip_and_then(&mut self, rhs: &Vec2<T>, f: F) {
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(rhs.x.iter())
            .zip(rhs.y.iter())
            .for_each(|((lhs, x), y)| f(lhs, *x, *y))
    }
}

impl<T: Copy, F: Fn(&mut T, T)> ZipAndThen<T, T, F> for Vec1<T> {
    fn zip_and_then(&mut self, rhs: T, f: F) {
        self.iter_mut().for_each(|value| f(value, rhs))
    }
}

impl<T: Default> Vec1<T> {
    pub fn default_with_len(len: usize) -> Self {
        Vec1 {
            values: Vec1::get_vec(len)
        }
    }

    fn get_vec(len: usize) -> Vec<T> {
        let mut values = Vec::with_capacity(len);
        values.extend(std::iter::repeat_with(|| T::default()).take(len));
        values
    }

    fn resize_with_default(&mut self, len: usize) {
        self.values.resize_with(len, || T::default());
    }
}

impl<T: Float + Default> Vec1<T> {
    pub fn get_magnitude_squared(&mut self, source: &Vec2<T>) {
        self.resize_with_default(source.len());
        self.zip_and_then(source, Self::calc_magnitude_squared);
    }

    fn calc_magnitude_squared(value: &mut T, x: T, y: T) {
        *value = (x * x) + (y * y);
    }

    pub fn get_magnitude(&mut self, source: &Vec2<T>) {
        self.resize_with_default(source.len());
        self.zip_and_then(source, Self::calc_magnitude);
    }

    fn calc_magnitude(value: &mut T, x: T, y: T) {
        *value = ((x * x) + (y * y)).sqrt();
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Self> for Vec1<T> {
    fn add_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::add_assign)
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<T> for Vec1<T> {
    fn add_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::add_assign)
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Self> for Vec1<T> {
    fn sub_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::sub_assign)
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<T> for Vec1<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::sub_assign)
    }
}

impl<T: Copy + MulAssign<T>> MulAssign<&Self> for Vec1<T> {
    fn mul_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::mul_assign)
    }
}

impl<T: Copy + MulAssign<T>> MulAssign<T> for Vec1<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::mul_assign)
    }
}

impl<T: Copy + DivAssign<T>> DivAssign<&Self> for Vec1<T> {
    fn div_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::div_assign)
    }
}

impl<T: Copy + DivAssign<T>> DivAssign<T> for Vec1<T> {
    fn div_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::div_assign)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: Vec1<T>,
    pub y: Vec1<T>,
}

impl<T> Default for Vec2<T> {
    fn default() -> Self {
        Vec2 {
            x: Vec1::default(),
            y: Vec1::default(),
        }
    }
}

impl<T> Vec2<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    fn iter(&self) -> impl Iterator<Item=&[T]> {
        once(self.x.as_slice()).chain(once(self.y.as_slice()))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut [T]> {
        once(self.x.as_mut_slice()).chain(once(self.y.as_mut_slice()))
    }
}

impl<T: Copy, F: Fn(&mut T, T)> ZipAndThen<&Vec2<T>, T, F> for Vec2<T> {
    fn zip_and_then(&mut self, rhs: &Vec2<T>, f: F) {
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| {
                lhs.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(lhs, rhs)| {
                        f(lhs, *rhs)
                    })
            })
    }
}

impl<T: Copy, F: Fn(&mut T, T)> ZipAndThen<&Vec1<T>, T, F> for Vec2<T> {
    fn zip_and_then(&mut self, rhs: &Vec1<T>, f: F) {
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(repeat(rhs.as_slice()))
            .for_each(|(lhs, rhs)| {
                lhs.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(lhs, rhs)| {
                        f(lhs, *rhs)
                    })
            })
    }
}

impl<T: Copy, F: Fn(&mut T, T)> ZipAndThen<T, T, F> for Vec2<T> {
    fn zip_and_then(&mut self, rhs: T, f: F) {
        self.iter_mut()
            .for_each(|values| {
                values.iter_mut()
                    .for_each(|v| f(v, rhs))
            })
    }
}

impl<T: Default> Vec2<T> {
    pub fn default_with_len(len: usize) -> Self {
        Vec2 {
            x: Vec1::default_with_len(len),
            y: Vec1::default_with_len(len),
        }
    }
}

impl<T: Float + AddAssign<T>> AddAssign<&Self> for Vec2<T> {
    fn add_assign(&mut self, rhs: &Vec2<T>) {
        self.zip_and_then(rhs, T::add_assign);
    }
}

impl<T: Float + SubAssign<T>> SubAssign<&Self> for Vec2<T> {
    fn sub_assign(&mut self, rhs: &Vec2<T>) {
        self.zip_and_then(rhs, T::sub_assign);
    }
}

impl<T: Float + MulAssign<T>> MulAssign<&Vec1<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::mul_assign);
    }
}

impl<T: Float + MulAssign<T>> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::mul_assign);
    }
}

impl<T: Float + DivAssign<T>> DivAssign<&Vec1<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_and_then(rhs, T::div_assign);
    }
}

impl<T: Float + DivAssign<T>> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.zip_and_then(rhs, T::div_assign);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign_vec1_to_vec1() {
        let mut v1 = Vec1 {
            values: vec![1.0, 2.0],
        };

        let v2 = Vec1 {
            values: vec![7.0, 11.0],
        };

        let expected = Vec1 {
            values: vec![8.0, 13.0],
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

        v1.zip_and_then(&v2, |v, x, y| *v += (x * x + y * y).sqrt());

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
}
