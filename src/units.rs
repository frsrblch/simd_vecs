use std::ops::*;

pub trait Unit {
    fn symbol() -> Option<&'static str>;
}

impl Unit for () {
    fn symbol() -> Option<&'static str> {
        None
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Meters;

impl Unit for Meters {
    fn symbol() -> Option<&'static str> {
        Some("m")
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Seconds;

impl Unit for Seconds {
    fn symbol() -> Option<&'static str> {
        Some("s")
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MetersPerSecond;

impl Unit for MetersPerSecond {
    fn symbol() -> Option<&'static str> {
        Some("m/s")
    }
}

impl Div<Seconds> for Meters {
    type Output = MetersPerSecond;

    fn div(self, _: Seconds) -> Self::Output {
        Default::default()
    }
}

impl Mul<Seconds> for MetersPerSecond {
    type Output = Meters;

    fn mul(self, _: Seconds) -> Self::Output {
        Default::default()
    }
}