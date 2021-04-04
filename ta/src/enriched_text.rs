use crate::zone::*;

#[derive(Clone, PartialEq)]
pub struct EnrichedText<T> {
    pub content: String,
    pub zones: Vec<Zone<T>>,
}

impl<T> EnrichedText<T> {
    pub fn new(text: String) -> Self {
        Self {
            content: text,
            zones: vec![],
        }
    }

    pub fn add(&mut self, zone: Zone<T>) {
        self.zones.push(zone)
    }

    fn all_zones_endpoints_sorted(&self) -> Vec<ZoneEndpoint> {
        let mut all_endpoints: Vec<ZoneEndpoint> = Vec::new();
        for i in 0..self.zones.len() {
            for interval in self.zones[i].intervals.clone() {
                all_endpoints.push(ZoneEndpoint::Beg(interval.beg, i));
                all_endpoints.push(ZoneEndpoint::End(interval.end, i));
            }
        }
        use ZoneEndpoint::*;
        all_endpoints.sort_by_key(|endpoint| match endpoint.clone() {
            Beg(pos, zone) | End(pos, zone) => (pos, zone),
        });
        // use lexicographical ordre on position,zone
        all_endpoints
    }

    // TODO implement an iterator for that
    // TODO give it nice types
    pub fn partition_by_zones(&self) -> Vec<(usize, Vec<usize>)> {
        let all_endpoints_sorted = self.all_zones_endpoints_sorted();
        use ZoneEndpoint::*;
        let mut res: Vec<(usize, Vec<usize>)> = Vec::new();
        let mut last_pos: usize = 0;
        let mut zones: Vec<usize> = Vec::new();
        for endpoint in all_endpoints_sorted.iter() {
            match endpoint {
                Beg(pos, zone) => {
                    if *pos == last_pos {
                        zones.push(*zone);
                        last_pos = *pos;
                    } else {
                        res.push((*pos - last_pos, zones.clone()));
                        zones.push(*zone);
                        last_pos = *pos;
                    }
                }
                End(pos, zone) => {
                    if *pos == last_pos {
                        zones.retain(|z| z != zone);
                        last_pos = *pos;
                    } else {
                        res.push((*pos - last_pos, zones.clone()));
                        zones.retain(|z| z != zone);
                        last_pos = *pos;
                    }
                }
            }
        }
        if let Some(endpoint) = all_endpoints_sorted.last() {
            match endpoint {
                Beg(pos, _) | End(pos, _) => {
                    let char_count = self.content.chars().count();
                    if *pos != char_count {
                        res.push((char_count - *pos, Vec::new()))
                    }
                }
            }
        }
        res
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ZoneEndpoint {
    Beg(usize, usize), // position, zone
    End(usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_zones_endpoints_sorted() {
        use ZoneEndpoint::*;
        let text : String = "Nulla varius lacus enim, a ullamcorper enim eleifend nec. Suspendisse vel nisl id mauris consequat luctus. Integer laoreet posuere accumsan. Vivamus nec sollicitudin velit, quis eleifend mauris. Vivamus ut nulla efficitur, laoreet justo ac, iaculis eros. Aliquam sed laoreet eros. Nulla at aliquet ipsum. Quisque eleifend vestibulum dolor et vehicula. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Praesent id vulputate eros, non congue nisi. Vivamus lorem erat, pharetra et turpis vitae, euismod dignissim ex. Proin ornare sem a massa feugiat, in aliquet quam aliquet.".into();
        let zone0: Zone<String> = vec![2..8, 50..120, 135..160, 500..550].into();
        let zone1: Zone<String> = vec![0..7, 12..306, 400..401].into();
        let zone2: Zone<String> = vec![2..8, 40..306, 400..406].into();

        let mut et = EnrichedText::new(text);
        et.add(zone0);
        et.add(zone1);
        et.add(zone2);

        let zones_sorted_endpoints = et.all_zones_endpoints_sorted();
        let zones_sorted_endpoints_ref = vec![
            Beg(0, 1),
            Beg(2, 0),
            Beg(2, 2),
            End(7, 1),
            End(8, 0),
            End(8, 2),
            Beg(12, 1),
            Beg(40, 2),
            Beg(50, 0),
            End(120, 0),
            Beg(135, 0),
            End(160, 0),
            End(306, 1),
            End(306, 2),
            Beg(400, 1),
            Beg(400, 2),
            End(401, 1),
            End(406, 2),
            Beg(500, 0),
            End(550, 0),
        ];

        assert_eq!(zones_sorted_endpoints_ref, zones_sorted_endpoints);
    }

    #[test]
    fn test_partition_by_zones() {
        use ZoneEndpoint::*;
        let text : String = "Nulla varius lacus enim, a ullamcorper enim eleifend nec. Suspendisse vel nisl id mauris consequat luctus. Integer laoreet posuere accumsan. Vivamus nec sollicitudin velit, quis eleifend mauris. Vivamus ut nulla efficitur, laoreet justo ac, iaculis eros. Aliquam sed laoreet eros. Nulla at aliquet ipsum. Quisque eleifend vestibulum dolor et vehicula. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Praesent id vulputate eros, non congue nisi. Vivamus lorem erat, pharetra et turpis vitae, euismod dignissim ex. Proin ornare sem a massa feugiat, in aliquet quam aliquet.".into();
        let zone0: Zone<String> = vec![2..8, 50..120, 135..160, 500..550].into();
        let zone1: Zone<String> = vec![1..7, 12..306, 400..401].into();
        let zone2: Zone<String> = vec![2..8, 40..306, 400..406].into();

        let mut et = EnrichedText::new(text);
        et.add(zone0);
        et.add(zone1);
        et.add(zone2);

        let partition_by_zones = et.partition_by_zones();
        let partition_by_zones_ref: Vec<(usize, Vec<usize>)> = vec![
            (1, vec![]),
            (1, vec![1]),
            (5, vec![1, 0, 2]),
            (1, vec![0, 2]),
            (4, vec![]),
            (28, vec![1]),
            (10, vec![1, 2]),
            (70, vec![1, 2, 0]),
            (15, vec![1, 2]),
            (25, vec![1, 2, 0]),
            (146, vec![1, 2]),
            (94, vec![]),
            (1, vec![1, 2]),
            (5, vec![2]),
            (94, vec![]),
            (50, vec![0]),
            (60, vec![]),
        ];

        assert_eq!(partition_by_zones_ref, partition_by_zones);
    }
}
