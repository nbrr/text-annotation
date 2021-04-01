use crate::interval::*;

// A collection of non-overlapping intervals
#[derive(Clone)]
pub struct Zone<T> {
    intervals: Vec<Interval>,
    data: Option<T>,
}

impl<T> Zone<T> {
    fn new() -> Self {
        Zone {
            intervals: vec![],
            data: None,
        }
    }

    // FIXME: insert with order
    // FIXME error management
    fn add(&mut self, interval: Interval) -> bool {
        if !self.intervals.iter().any(|i| i.intersect(interval)) {
            self.intervals.push(interval);
            true
        } else {
            false
        }
    }

    fn set(&mut self, data: T) {
        self.data = Some(data)
    }
}
