use rand::seq::SliceRandom;

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
pub fn is_sorted(elements: &[u32]) -> bool {
    elements.windows(2).all(|w| w[0] <= w[1])
}
pub fn shuffle(elements: &mut [u32]) {
    let mut rng = rand::thread_rng();
    elements.shuffle(&mut rng);
}
