use crate::zone::*;

#[derive(Clone)]
struct EnrichedText<T> {
    content: String,
    zones: Vec<Zone<T>>,
}

impl<T> EnrichedText<T> {
    fn new(text: String) -> Self {
        Self {
            content: text,
            zones: vec![],
        }
    }

    fn add(&mut self, zone: Zone<T>) {
        self.zones.push(zone)
    }
}
