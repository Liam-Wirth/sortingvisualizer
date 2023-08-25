use crate::array::Array;
use crate::sorts::algorithm::{Algorithm, Complexity, Info};
use crate::sorts::is_sorted;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
pub struct BogoSort {
    array: Vec<u32>,
    index: usize,
    sorted: bool,
}

impl BogoSort {
    pub fn new(mut array: Vec<u32>) -> Self {
        BogoSort{
            array,
            index: 0,
            sorted: false,
        }
    }
}
/*
impl Algorithm for BogoSort {
    fn step(&mut self) -> Vec<u32> {
        if self.sorted {
            return self.array.clone()
        };
        self.array.shuffle(&mut thread_rng());
        self.array.clone()
    }
    
}
*/
