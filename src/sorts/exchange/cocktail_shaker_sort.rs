//NOTE: just now realizing that working on this old version of my app
//is practically a lost cause considering that I want to rebuild the way
//the app works from the ground up
use crate::array::Array;
use crate::sorts::algorithm::{Algorithm, Complexity, Info};
use crate::sorts::is_sorted;

pub struct CocktailShakerSort {
    pub index: usize,
    sorted: bool,
    max_index: usize,
    min_index: usize,
    direction: bool, //false = left to right 
                     //true = right to left
    swapped: bool,
}

impl CocktailShakerSort {
    pub fn new(len: usize) -> Self {
        CocktailShakerSort{
            //ideally use just an array of nums in the future? maybe 
            index: 0,
            max_index: len,
            sorted: false,
            min_index: 0,
            direction: false,

        }
    }
}
impl Algorithm for CocktailShakerSort {
    fn step(&mut self, elements: &mut [u32]) -> (bool, &usize) {
        
        if self.sorted {
            return (true, &self.index);
        };
        //left to right pass
        if (!self.direction) {
            if elements[self.index] > elements[self.index + 1] {
                let temp: u32 = elements[self.index];
                elements[self.index] = elements[self.index+1].clone();
                elements[self.index + 1] = temp;
                self.swapped = true;
            }
        }
        if !self.swapped {
            self.sorted = true;
        }
       return (false, &self.index);
    }
}
