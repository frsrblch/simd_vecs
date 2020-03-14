use super::*;

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

impl<'a, T: Copy + AddAssign<T> + Mul<T, Output=T>> AddAssign<VMul<'a, Vec2<T>, T>> for Vec2<T> {
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
