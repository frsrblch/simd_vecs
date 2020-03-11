#[derive(Debug, Copy, Clone)]
pub struct VMul<'a, A, B> {
    pub a: &'a A,
    pub b: &'a B,
}