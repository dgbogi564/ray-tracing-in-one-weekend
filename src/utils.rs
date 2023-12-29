pub(crate) fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
macro_rules! random_double {
    () => {{
        use rand::{Rng, thread_rng};
        let num : f64 = thread_rng().gen();
        num
    }};
    ($min:expr, $max:expr) => {{
        use rand::{Rng, thread_rng};
        let num : f64 = thread_rng().gen_range($min..$max);
        num
    }};
}

use rand::Rng;
pub(crate) use random_double;