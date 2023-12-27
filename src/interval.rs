pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub(crate) fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub(crate) fn default() -> Self {
        Interval { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }

    pub(crate) fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };