use crate::array::Array;
use crate::sorts::algorithm::{Algorithm, Complexity, Info};
use crate::sorts::{is_sorted, shuffle};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
pub struct BogoSort {
    pub index: usize,
    sorted: bool,
    max_index: usize,
}

impl BogoSort {
    pub fn new(len:usize) -> Self {
        BogoSort{
            index: 0,
            sorted: false,
            max_index: len,
            
        }
    }
}
impl Algorithm for BogoSort {
    fn step(&mut self, elements: &mut [u32]) -> (bool,&usize) {
        let mut rng = thread_rng();
         self.index = rng.gen_range(0..elements.len());
        //NOTE: I dunno about above tbh, I think returning a random index might help display that the algorithm is just randomizing shit
        if self.sorted {
            return (true, &self.index);
        } else {
            elements.shuffle(&mut rng);
            self.sorted = is_sorted(elements);
            return(self.sorted, &self.index);
        }

    }
    fn name(&self) -> String {
        String::from("Bogo Sort")
    }
   fn info(&self) -> Info {
    Info {
        name: self.name(),
        description: String::from("a highly inefficient sorting algorithm that works by repeatedly shuffling the elements of an array randomly until the array happens to be sorted."),
        complexity: Complexity::Unbounded,
        method: String::from("Randomize?"),
    }
   } 
}
