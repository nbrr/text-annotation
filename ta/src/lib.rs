mod enriched_text;
mod interval;
mod zone;

pub use crate::enriched_text::*;
pub use crate::interval::*;
pub use crate::zone::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
