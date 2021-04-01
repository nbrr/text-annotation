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
}
