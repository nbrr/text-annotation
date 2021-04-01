use boolinator::*;
use std::ops::Range;

/// Range of indices in a text.
// FIXME use std::ops::Range?
#[derive(Copy,Clone, PartialEq)]
pub struct Interval {
    beg: usize,
    end: usize,
}

impl Interval {
    //FIXME Interval should be related to a text, and thus endpoints ought to respect text properties
    pub fn new(beg: usize, end: usize) -> Option<Self> {
        (beg < end).as_some(Interval { beg, end })
    }

    pub fn contains(&self, ind: usize) -> bool {
        self.beg <= ind && ind <= self.end
    }

    pub fn intersect(&self, i: Self) -> bool {
        i.end < self.end || self.end < i.beg
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
