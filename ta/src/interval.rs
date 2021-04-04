use boolinator::*;
use std::ops::Range;

/// Range of indices in a text.
// FIXME use std::ops::Range?
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Interval {
    pub beg: usize,
    pub end: usize,
}

impl Interval {
    //FIXME Interval should be related to a text, and thus endpoints ought to respect text properties
    pub fn new(beg: usize, end: usize) -> Option<Self> {
        (beg < end).as_some(Interval { beg, end })
    }

    pub fn contains(&self, ind: usize) -> bool {
        self.beg <= ind && ind <= self.end
    }

    pub fn intersects(&self, interval: Self) -> bool {
        self.contains(interval.beg)
            || self.contains(interval.end)
            || interval.contains(self.beg)
            || interval.contains(self.end)
    }
}

// FIXME TryFrom
impl From<&Range<usize>> for Interval {
    fn from(range: &Range<usize>) -> Self {
        Interval {
            beg: range.start,
            end: range.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        let interval_2_8 = Interval::new(2, 8).unwrap();
        let interval_4_10 = Interval::new(4, 10).unwrap();
        let interval_8_12 = Interval::new(8, 12).unwrap();
        let interval_1_16 = Interval::new(1, 16).unwrap();
        let interval_20_40 = Interval::new(20, 40).unwrap();

        assert_eq!(interval_2_8.intersects(interval_4_10), true);
        assert_eq!(interval_4_10.intersects(interval_2_8), true);

        assert_eq!(interval_2_8.intersects(interval_8_12), true);
        assert_eq!(interval_8_12.intersects(interval_2_8), true);

        assert_eq!(interval_4_10.intersects(interval_20_40), false);
        assert_eq!(interval_20_40.intersects(interval_4_10), false);

        assert_eq!(interval_4_10.intersects(interval_1_16), true);
        assert_eq!(interval_1_16.intersects(interval_4_10), true);
    }
}
