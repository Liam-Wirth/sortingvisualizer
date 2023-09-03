
use crate::sorts::algorithm::{Algorithm, Complexity, Info};
use crate::sorts::{is_sorted, remove_element_at_index};

use std::mem;
pub struct StalinSort {
    sorted: bool,
    pub index: usize,
}

impl StalinSort {
    pub fn new() -> Self {
        StalinSort {
            sorted: false,
            index: 0,
        }
    }
}
/*
//TODO: This is broken right now, I cant adjust the size of the given array slice
impl Algorithm for StalinSort {
    fn step(&mut self, elements: &mut [u32]) -> (bool, &usize) {
        if self.index == elements.len() {
            //NOTE: This algorithm is done in one pass cause it's a joke algorithm so if I'm at the
            //max index, then the array literally HAS to be sorted
            return (true, &self.index);
        }
        if elements.get(self.index) > elements.get(self.index + 1) {
            println!("{:?}", elements);
            let mut new_elements = elements.to_vec();
            new_elements.remove(self.index);
            println!("{:?}", elements);
        };
        self.index += 1;

        (self.sorted, &self.index)
    }
    fn name(&self) -> String {
        String::from("Bubble Sort")
    }
}
*/
