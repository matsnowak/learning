// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const MULTIPLIER: f64;
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64 / 31_557_600_f64) / Self::MULTIPLIER
    }
}
pub struct Mercury;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
pub struct Venus;

macro_rules! impl_Planet {
    ($planet_struct:ident, $multiplier:expr) => {
        impl Planet for $planet_struct {
            const MULTIPLIER: f64 = $multiplier;
        }
    };
}

impl_Planet!(Mercury, 0.2408467);
impl_Planet!(Venus, 0.61519726);
impl_Planet!(Earth, 1.0);
impl_Planet!(Mars, 1.8808158);
impl_Planet!(Jupiter, 11.862615);
impl_Planet!(Saturn, 29.447498);
impl_Planet!(Uranus, 84.016846);
impl_Planet!(Neptune, 164.79132);
