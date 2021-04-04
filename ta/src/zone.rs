use crate::interval::*;
use std::ops::Range;

// A collection of non-overlapping intervals
#[derive(Clone, PartialEq, Debug)]
pub struct Zone<T> {
    pub intervals: Vec<Interval>,
    pub data: Option<T>,
}

impl<T> Zone<T> {
    pub fn new() -> Self {
        Zone {
            intervals: vec![],
            data: None,
        }
    }

    // FIXME error management
    // TODO merge intersecting intervals
    pub fn insert(&mut self, interval: Interval) -> bool {
        if !self.intervals.iter().any(|i| i.intersects(interval)) {
            if let Some((index, _)) = self
                .intervals
                .iter()
                .enumerate()
                .find(|(_, int)| int.beg > interval.beg)
            {
                self.intervals.insert(index, interval)
            } else {
                self.intervals.push(interval)
            }
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, data: T) {
        self.data = Some(data)
    }

    pub fn contains(&self, ind: usize) -> bool {
        self.intervals.iter().any(|i| i.contains(ind))
    }
}

// FIXME TryFrom
impl<T> From<&Range<usize>> for Zone<T> {
    fn from(range: &Range<usize>) -> Self {
        Self {
            intervals: vec![Interval::from(range)],
            data: None,
        }
    }
}

// FIXME TryFrom
impl<T> From<Vec<&Range<usize>>> for Zone<T> {
    fn from(ranges: Vec<&Range<usize>>) -> Self {
        Self {
            // FIXME how to write without deref?
            intervals: ranges.iter().map(|r| Interval::from(*r)).collect(),
            data: None,
        }
    }
}

// FIXME TryFrom
impl<T> From<Vec<Range<usize>>> for Zone<T> {
    fn from(ranges: Vec<Range<usize>>) -> Self {
        Self {
            // FIXME how to write without deref?
            intervals: ranges.iter().map(Interval::from).collect(),
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let interval_2_8 = Interval::new(2, 8).unwrap();
        let interval_4_10 = Interval::new(4, 10).unwrap();
        let interval_8_12 = Interval::new(8, 12).unwrap();
        let interval_20_40 = Interval::new(20, 40).unwrap();
        let interval_55_76 = Interval::new(55, 76).unwrap();

        let mut zone = Zone::<()>::new();

        assert_eq!(zone.insert(interval_8_12), true);
        assert_eq!(zone.insert(interval_2_8), false);

        assert_eq!(zone.insert(interval_55_76), true);
        assert_eq!(zone.intervals[0], interval_8_12);
        assert_eq!(zone.intervals[1], interval_55_76);

        assert_eq!(zone.insert(interval_20_40), true);
        assert_eq!(zone.insert(interval_4_10), false);
        assert_eq!(zone.intervals[0], interval_8_12);
        assert_eq!(zone.intervals[1], interval_20_40);
        assert_eq!(zone.intervals[2], interval_55_76);
    }
}
