pub fn space_age() {}

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const BASE_TIME:f64 = 31557600.0; 
const MERCURY:f64 = 0.2408467;
const VENUS:f64 = 0.61519726;
const MARS:f64 = 1.8808158;
const JUPITER:f64 = 11.862615;
const SATURN:f64 = 29.447498;
const URANUS:f64 = 84.016846;
const NEPTUNE:f64 = 164.79132;

// #[derive(Debug)]
// pub struct Duration(u64);

// impl From<u64> for Duration {
//     fn from(s: u64) -> Self {
//         Self(s)
//     }
// }

// pub trait Planet {
//     fn years_during(d: &Duration) -> f64;
// }

// pub struct Mercury;
// pub struct Venus;
// pub struct Earth;
// pub struct Mars;
// pub struct Jupiter;
// pub struct Saturn;
// pub struct Uranus;
// pub struct Neptune;

// impl Planet for Mercury {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / MERCURY
//     }
// }
// impl Planet for Venus {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / VENUS
//     }
// }
// impl Planet for Earth {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME
//     }
// }
// impl Planet for Mars {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / MARS
//     }
// }
// impl Planet for Jupiter {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / JUPITER
//     }
// }
// impl Planet for Saturn {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / SATURN
//     }
// }
// impl Planet for Uranus {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / URANUS
//     }
// }
// impl Planet for Neptune {
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 as f64) / BASE_TIME / NEPTUNE
//     }
// }
pub struct Duration(u64);

impl From<u64> for Duration{
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64{
        (d.0 as f64) / BASE_TIME/ Self::period()
    }
}

macro_rules! impl_planet {
    ($(($n: ident, $p: expr)),*) => {
        $(
            pub struct $n;
            impl Planet for $n{
                fn period() -> f64{
                    $p
                }
            }
        )*
    };
}

impl_planet!(
    (Earth, 1.0),
    (Mercury, MERCURY),
    (Venus, VENUS),
    (Mars, MARS),
    (Jupiter, JUPITER),
    (Saturn, SATURN),
    (Uranus, URANUS),
    (Neptune, NEPTUNE)
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!(
                "Your result of {} should be within {} of the expected result {}",
                actual, delta, expected
            )
        }
    }
    #[test]
    fn earth_age() {
        let duration = Duration::from(1_000_000_000);
        assert_in_delta(31.69, Earth::years_during(&duration));
    }
    #[test]
    fn mercury_age() {
        let duration = Duration::from(2_134_835_688);
        assert_in_delta(280.88, Mercury::years_during(&duration));
    }
    #[test]
    fn venus_age() {
        let duration = Duration::from(189_839_836);
        assert_in_delta(9.78, Venus::years_during(&duration));
    }
    #[test]
    fn mars_age() {
        let duration = Duration::from(2_129_871_239);
        assert_in_delta(35.88, Mars::years_during(&duration));
    }
    #[test]
    fn jupiter_age() {
        let duration = Duration::from(901_876_382);
        assert_in_delta(2.41, Jupiter::years_during(&duration));
    }
    #[test]
    fn saturn_age() {
        let duration = Duration::from(2_000_000_000);
        assert_in_delta(2.15, Saturn::years_during(&duration));
    }
    #[test]
    fn uranus_age() {
        let duration = Duration::from(1_210_123_456);
        assert_in_delta(0.46, Uranus::years_during(&duration));
    }
    #[test]
    fn neptune_age() {
        let duration = Duration::from(1_821_023_456);
        assert_in_delta(0.35, Neptune::years_during(&duration));
    }
}
