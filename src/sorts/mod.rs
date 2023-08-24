pub mod algorithm;
pub mod concurrent;
pub mod distribute;
pub mod exchange;
pub mod hybrid;
pub mod insert;
pub mod merge;
pub mod misc;
pub mod quick;
pub mod select;
pub fn is_sorted(elements: &Vec<u32>) -> bool {
        elements.windows(2).all(|w| w[0] <= w[1])
    }

