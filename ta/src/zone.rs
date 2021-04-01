use crate::interval::*;
use std::ops::Range;

// A collection of non-overlapping intervals
#[derive(Clone, PartialEq)]
pub struct Zone<T> {
    intervals: Vec<Interval>,
    data: Option<T>,
}

impl<T> Zone<T> {
    pub fn new() -> Self {
        Zone {
            intervals: vec![],
            data: None,
        }
    }

    // FIXME: insert with order
    // FIXME error management
    pub fn add(&mut self, interval: Interval) -> bool {
        if !self.intervals.iter().any(|i| i.intersect(interval)) {
            self.intervals.push(interval);
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, data: T) {
        self.data = Some(data)
    }

    pub fn contains(&self, ind: usize)-> bool {
        self.intervals.iter().any(|i| i.contains(ind))
    } 
}

// FIXME TryFrom
impl<T> From<&Range<usize>> for Zone<T> {
    fn from(range: &Range<usize>) -> Self {
        Self {
            intervals : vec![Interval::from(range)],
            data: None,
        }
    }
}

// FIXME TryFrom
impl<T> From<Vec<&Range<usize>>> for Zone<T> {
    fn from(ranges: Vec<&Range<usize>>) -> Self {
        Self {
            // FIXME how to write without deref?
            intervals : ranges.iter().map(|r| Interval::from(*r)).collect(),
            data: None,
        }
    }
}

// FIXME TryFrom
impl<T> From<Vec<Range<usize>>> for Zone<T> {
    fn from(ranges: Vec<Range<usize>>) -> Self {
        Self {
            // FIXME how to write without deref?
            intervals : ranges.iter().map(Interval::from).collect(),
            data: None,
        }
    }
}
