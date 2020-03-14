use super::*;

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

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
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
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(v, r)| f(v, *r));
    }

    pub fn zip_to_vec2<T2: Copy, F: Fn(&mut T, T2, T2)>(&mut self, rhs: &Vec2<T2>, f: F) {
        debug_assert_eq!(self.len(), rhs.len());

        self.iter_mut()
            .zip(rhs.x.iter())
            .zip(rhs.y.iter())
            .for_each(|((v, x), y)| f(v, *x, *y));
    }

    pub fn zip_to_vec1_and_vec1<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, a: &Vec1<T2>, b: &Vec1<T3>, f: F) {
        debug_assert_eq!(self.len(), a.len());
        debug_assert_eq!(self.len(), b.len());

        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((v, a), b)| f(v, *a, *b));
    }

    pub fn zip_to_vec1_and_value<T2: Copy, T3: Copy, F: Fn(&mut T, T2, T3)>(&mut self, a: &Vec1<T2>, b: T3, f: F) {
        debug_assert_eq!(self.len(), a.len());

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

impl<'a, T1: Copy + AddAssign<T1>, T2: Copy + Mul<T3, Output=T1>, T3: Copy> AddAssign<VMul<'a, Vec1<T2>, Vec1<T3>>> for Vec1<T1> {
    fn add_assign(&mut self, rhs: VMul<'a, Vec1<T2>, Vec1<T3>>) {
        self.zip_to_vec1_and_vec1(rhs.a, rhs.b, |a, b, c| *a += b * c);
    }
}

impl<'a, T1: Copy + AddAssign<T1>, T2: Copy + Mul<T3, Output=T1>, T3: Copy> AddAssign<VMul<'a, Vec1<T2>, T3>> for Vec1<T1> {
    fn add_assign(&mut self, rhs: VMul<'a, Vec1<T2>, T3>) {
        self.zip_to_vec1_and_value(rhs.a, *rhs.b, |a, b, c| *a += b * c);
    }
}

impl<'a, T1: Copy + SubAssign<T1>, T2: Copy + Mul<T3, Output=T1>, T3: Copy> SubAssign<VMul<'a, Vec1<T2>, Vec1<T3>>> for Vec1<T1> {
    fn sub_assign(&mut self, rhs: VMul<'a, Vec1<T2>, Vec1<T3>>) {
        self.zip_to_vec1_and_vec1(rhs.a, rhs.b, |a, b, c| *a -= b * c);
    }
}

impl<'a, T1: Copy + SubAssign<T1>, T2: Copy + Mul<T3, Output=T1>, T3: Copy> SubAssign<VMul<'a, Vec1<T2>, T3>> for Vec1<T1> {
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
