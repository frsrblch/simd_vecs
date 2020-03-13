use crate::ops::*;
use num_traits::Float;
use std::ops::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec1<T> {
    pub values: Vec<T>,
}

impl<T> Default for Vec1<T> {
    fn default() -> Self {
        Vec1 { values: vec![] }
    }
}

impl<T> From<Vec<T>> for Vec1<T> {
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<T> Vec1<T> {
    pub fn new() -> Self {
        Vec1 { values: vec![] }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[T] {
        self.values.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.values.as_mut_slice()
    }

    pub fn zip_to_value<T2: Copy, F: Fn(&mut T, T2)>(&mut self, rhs: T2, f: F) {
        self.iter_mut()
            .for_each(|v| f(v, rhs));
    }

    pub fn zip_to_vec1<T2: Copy, F: Fn(&mut T, T2)>(&mut self, rhs: &Vec1<T2>, f: F) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(v, r)| f(v, *r));
    }

    pub fn zip_to_vec2<T2: Copy, F: Fn(&mut T, T2, T2)>(&mut self, rhs: &Vec2<T2>, f: F) {
        self.iter_mut()
            .zip(rhs.x.iter())
            .zip(rhs.y.iter())
            .for_each(|((v, x), y)| f(v, *x, *y));
    }

    pub fn zip_to_vec1_and_vec1<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, a: &Vec1<T2>, b: &Vec1<T3>, f: F) {
        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((v, a), b)| f(v, *a, *b));
    }

    pub fn zip_to_vec1_and_value<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, a: &Vec1<T2>, b: T3, f: F) {
        self.iter_mut()
            .zip(a.iter())
            .for_each(|(v, a)| f(v, *a, b));
    }
}

impl<T: Default> Vec1<T> {
    pub fn default_with_len(len: usize) -> Self {
        Vec1 {
            values: Vec1::get_vec(len),
        }
    }

    fn get_vec(len: usize) -> Vec<T> {
        let mut values = Vec::with_capacity(len);
        values.extend(std::iter::repeat_with(T::default).take(len));
        values
    }

    fn resize_with_default(&mut self, len: usize) {
        self.values.resize_with(len, T::default);
    }
}

impl<T: Float + Default> Vec1<T> {
    pub fn get_magnitude_squared(&mut self, source: &Vec2<T>) {
        self.resize_with_default(source.len());
        self.zip_to_vec2(source, Self::calc_magnitude_squared);
    }

    fn calc_magnitude_squared(value: &mut T, x: T, y: T) {
        *value = (x * x) + (y * y);
    }

    pub fn get_magnitude(&mut self, source: &Vec2<T>) {
        self.resize_with_default(source.len());
        self.zip_to_vec2(source, Self::calc_magnitude);
    }

    fn calc_magnitude(value: &mut T, x: T, y: T) {
        *value = ((x * x) + (y * y)).sqrt();
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Self> for Vec1<T> {
    fn add_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::add_assign)
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<T> for Vec1<T> {
    fn add_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::add_assign)
    }
}

impl<'a, T1: Copy + AddAssign<T1>, T2: Copy + Mul<T3,Output=T1>, T3: Copy> AddAssign<VMul<'a, Vec1<T2>, Vec1<T3>>> for Vec1<T1> {
    fn add_assign(&mut self, rhs: VMul<'a, Vec1<T2>, Vec1<T3>>) {
        self.zip_to_vec1_and_vec1(rhs.a, rhs.b, |a, b, c| *a += b * c);
    }
}

impl<'a, T1: Copy + AddAssign<T1>, T2: Copy + Mul<T3,Output=T1>, T3: Copy> AddAssign<VMul<'a, Vec1<T2>, T3>> for Vec1<T1> {
    fn add_assign(&mut self, rhs: VMul<'a, Vec1<T2>, T3>) {
        self.zip_to_vec1_and_value(rhs.a, *rhs.b, |a, b, c| *a += b * c);
    }
}

impl<'a, T1: Copy + SubAssign<T1>, T2: Copy + Mul<T3,Output=T1>, T3: Copy> SubAssign<VMul<'a, Vec1<T2>, Vec1<T3>>> for Vec1<T1> {
    fn sub_assign(&mut self, rhs: VMul<'a, Vec1<T2>, Vec1<T3>>) {
        self.zip_to_vec1_and_vec1(rhs.a, rhs.b, |a, b, c| *a -= b * c);
    }
}

impl<'a, T1: Copy + SubAssign<T1>, T2: Copy + Mul<T3,Output=T1>, T3: Copy> SubAssign<VMul<'a, Vec1<T2>, T3>> for Vec1<T1> {
    fn sub_assign(&mut self, rhs: VMul<'a, Vec1<T2>, T3>) {
        self.zip_to_vec1_and_value(rhs.a, *rhs.b, |a, b, c| *a -= b * c);
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Self> for Vec1<T> {
    fn sub_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::sub_assign)
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<T> for Vec1<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::sub_assign)
    }
}

impl<T: Copy + MulAssign<T>> MulAssign<&Self> for Vec1<T> {
    fn mul_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::mul_assign)
    }
}

impl<T: Copy + MulAssign<T>> MulAssign<T> for Vec1<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::mul_assign)
    }
}

impl<T: Copy + DivAssign<T>> DivAssign<&Self> for Vec1<T> {
    fn div_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::div_assign)
    }
}

impl<T: Copy + DivAssign<T>> DivAssign<T> for Vec1<T> {
    fn div_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::div_assign)
    }
}

impl<'a, T1: Mul<T2>, T2> Mul<&'a Vec1<T2>> for &'a Vec2<T1> {
    type Output = VMul<'a, Vec2<T1>, Vec1<T2>>;

    fn mul(self, rhs: &'a Vec1<T2>) -> Self::Output {
        VMul { a: self, b: rhs }
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

impl<T: Default> Vec2<T> {
    pub fn default_with_len(len: usize) -> Self {
        Vec2 {
            x: Vec1::default_with_len(len),
            y: Vec1::default_with_len(len),
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn zip_to_value<T2: Copy, F: Fn(&mut T, T2) + 'static>(&mut self, rhs: T2, f: F) {
        self.x.iter_mut().for_each(|x| f(x, rhs));
        self.y.iter_mut().for_each(|x| f(x, rhs));
    }

    pub fn zip_both_to_value<T2: Copy, F: Fn(&mut T, &mut T, T2) + 'static>(&mut self, rhs: T2, f: F) {
        self.x.iter_mut()
            .zip(self.y.iter_mut())
            .for_each(|(x, y)| f(x, y, rhs));
    }

    pub fn zip_to_vec1<T2: Copy, F: Fn(&mut T, T2)>(&mut self, rhs: &Vec1<T2>, f: F) {
        self.x.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, r)| f(l, *r));

        self.y.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, r)| f(l, *r));
    }

    pub fn zip_both_to_vec1<T2: Copy, F: Fn(&mut T, &mut T, T2)>(&mut self, rhs: &Vec1<T2>, f: F) {
        self.x.iter_mut()
            .zip(self.y.iter_mut())
            .zip(rhs.iter())
            .for_each(|((x, y), r)| f(x, y, *r));
    }

    pub fn zip_to_vec2<T2: Copy, F: Fn(&mut T, T2)>(&mut self, rhs: &Vec2<T2>, f: F) {
        self.x.iter_mut()
            .zip(rhs.x.iter())
            .for_each(|(l, r)| f(l, *r));

        self.y.iter_mut()
            .zip(rhs.y.iter())
            .for_each(|(l, r)| f(l, *r));
    }

    pub fn zip_both_to_vec2<T2: Copy, F: Fn(&mut T, &mut T, T2, T2)>(&mut self, rhs: &Vec2<T2>, f: F) {
        self.x.iter_mut()
            .zip(self.y.iter_mut())
            .zip(rhs.x.iter())
            .zip(rhs.y.iter())
            .for_each(|(((x1, y1), x2), y2)| f(x1, y1, *x2, *y2));
    }

    pub fn zip_to_vec2_and_value<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, vec2: &Vec2<T2>, value: T3, f: F) {
        self.x.iter_mut()
            .zip(vec2.x.iter())
            .for_each(|(x1, x2)| f(x1, *x2, value));

        self.y.iter_mut()
            .zip(vec2.y.iter())
            .for_each(|(y1, y2)| f(y1, *y2, value));
    }

    pub fn zip_to_vec2_and_vec1<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, vec2: &Vec2<T2>, vec1: &Vec1<T3>, f: F) {
        self.x.iter_mut()
            .zip(vec2.x.iter())
            .zip(vec1.iter())
            .for_each(|((x1, x2), v)| f(x1, *x2, *v));

        self.y.iter_mut()
            .zip(vec2.y.iter())
            .zip(vec1.iter())
            .for_each(|((y1, y2), v)| f(y1, *y2, *v));
    }

    pub fn zip_both_to_vec2_and_value<T2: Copy, T3: Copy, F: Fn(&mut T, &mut T, T2, T2, T3)>(&mut self, vec2: &Vec2<T2>, value: T3, f: F) {
        self.x.iter_mut()
            .zip(self.y.iter_mut())
            .zip(vec2.x.iter())
            .zip(vec2.y.iter())
            .for_each(|(((x1, y1), x2), y2)| f(x1, y1, *x2, *y2, value));
    }

    pub fn zip_both_to_vec2_and_vec1<T2: Copy, T3: Copy, F: Fn(&mut T, &mut T, T2, T2, T3)>(&mut self, vec2: &Vec2<T2>, vec1: &Vec1<T3>, f: F) {
        self.x.iter_mut()
            .zip(self.y.iter_mut())
            .zip(vec2.x.iter())
            .zip(vec2.y.iter())
            .zip(vec1.iter())
            .for_each(|((((x1, y1), x2), y2), v)| f(x1, y1, *x2, *y2, *v));
    }
}

impl<'a, T1, T2, T3> AddAssign<VMul<'a, Vec2<T2>, Vec1<T3>>> for Vec2<T1>
    where
        T1: Copy + AddAssign<T1>,
        T2: Mul<T3, Output=T1> + Copy,
        T3: Copy,
{
    fn add_assign(&mut self, rhs: VMul<'a, Vec2<T2>, Vec1<T3>>) {
        self.zip_to_vec2_and_vec1(rhs.a, rhs.b, |a, b, c| *a += b.mul(c));
    }
}

impl<'a, T1, T2, T3> AddAssign<VDiv<'a, Vec2<T2>, Vec1<T3>>> for Vec2<T1>
    where
        T1: Copy + AddAssign<T1>,
        T2: Div<T3, Output=T1> + Copy,
        T3: Copy,
{
    fn add_assign(&mut self, rhs: VDiv<'a, Vec2<T2>, Vec1<T3>>) {
        self.zip_to_vec2_and_vec1(rhs.a, rhs.b, |a, b, c| *a += b.div(c));
    }
}

impl<'a, T1, T2, T3> SubAssign<VMul<'a, Vec2<T2>, Vec1<T3>>> for Vec2<T1>
    where
        T1: Copy + SubAssign<T1>,
        T2: Mul<T3, Output=T1> + Copy,
        T3: Copy,
{
    fn sub_assign(&mut self, rhs: VMul<'a, Vec2<T2>, Vec1<T3>>) {
        self.zip_to_vec2_and_vec1(rhs.a, rhs.b, |a, b, c| *a -= b.mul(c));
    }
}

impl<'a, T1, T2, T3> SubAssign<VDiv<'a, Vec2<T2>, Vec1<T3>>> for Vec2<T1>
    where
        T1: Copy + SubAssign<T1>,
        T2: Div<T3, Output=T1> + Copy,
        T3: Copy,
{
    fn sub_assign(&mut self, rhs: VDiv<'a, Vec2<T2>, Vec1<T3>>) {
        self.zip_to_vec2_and_vec1(rhs.a, rhs.b, |a, b, c| *a -= b.div(c));
    }
}

impl<T: Copy + AddAssign<T>> AddAssign<&Self> for Vec2<T> {
    fn add_assign(&mut self, rhs: &Vec2<T>) {
        self.zip_to_vec2(rhs, T::add_assign);
    }
}

impl<'a, T: Copy + AddAssign<T> + Mul<T, Output = T>> AddAssign<VMul<'a, Vec2<T>, T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: VMul<'a, Vec2<T>, T>) {
        self.zip_to_vec2_and_value(rhs.a, *rhs.b, |a, b, c| *a += b.mul(c));
    }
}

impl<T: Copy + SubAssign<T>> SubAssign<&Self> for Vec2<T> {
    fn sub_assign(&mut self, rhs: &Vec2<T>) {
        self.zip_to_vec2(rhs, T::sub_assign);
    }
}

impl<T: Copy + MulAssign<T>> MulAssign<&Vec1<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::mul_assign);
    }
}

impl<T: Copy + MulAssign<T> + 'static> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::mul_assign);
    }
}

impl<T: Copy + DivAssign<T>> DivAssign<&Vec1<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: &Vec1<T>) {
        self.zip_to_vec1(rhs, T::div_assign);
    }
}

impl<T: Copy + DivAssign<T> + 'static> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.zip_to_value(rhs, T::div_assign);
    }
}

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
