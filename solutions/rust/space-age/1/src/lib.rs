// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn period_in_earth_years() -> f64;
    fn years_during(d: &Duration) -> f64 {
        let earth_year_in_seconds = 31_557_600.0;
        let year_on_earth = d.seconds as f64 / earth_year_in_seconds;
        let period = Self::period_in_earth_years();
        year_on_earth / period
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn period_in_earth_years() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn period_in_earth_years() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn period_in_earth_years() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn period_in_earth_years() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn period_in_earth_years() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn period_in_earth_years() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn period_in_earth_years() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn period_in_earth_years() -> f64 {
        164.79132
    }
}
