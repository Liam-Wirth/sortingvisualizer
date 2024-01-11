// Import necessary modules
use crate::array::Array;
use crate::sorts::algorithm::{Algorithm, Complexity, Info};
use crate::sorts::is_sorted;

// Define the BubbleSort struct that implements Algorithm trait
//thinking about how this works, it'd make more sense right now at least for bubble sort to be
//called once every frame count, so I want to think about how I could preserve the state of things
//so that the sort only runs once per call, but maintains it's progress.
//
pub struct BubbleSort {
    //Keep the state/step it is on (probably takes ownership of this right?)
    //array: Vec<u32>,
    sorted: bool,
    max_index: usize,
}
//TODO: Make sure that this exists within it's own thread
impl BubbleSort {
    pub fn new(len: usize) -> Self {
        //println!("{len}");
        BubbleSort {
            //array,
            max_index: len,
            sorted: false,
        }
    }
}
impl Algorithm for BubbleSort {
    fn sort(&self, array: Array) -> bool {
        while(!self.sorted){
            
        }
        return true;
    }
    fn name(&self) -> String {
        String::from("Bubble Sort")
    }

    fn info(&self) -> Info {
        Info {
            name: self.name(),
            description: String::from("A simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order."),
            complexity: Complexity::Quadratic,
            method: String::from("bubble::sort"),
        }
    }
}
/*
impl Algorithm for BubbleSort {
    fn step(&mut self, elements: &mut [u32]) -> (bool, &usize) {
        if self.sorted {
            return (true, &self.index);
        }
        if self.index >= self.max_index - 1 {
            self.max_index -= 1;
            //check to see if it is sorted
            if is_sorted(elements) {
                return (true, &self.index);
            } else {
                self.index = 0;
            };
        };
        if elements.get(self.index) > elements.get(self.index + 1) {
            elements.swap(self.index, self.index + 1);
        };
        self.index += 1;
        self.sorted = is_sorted(elements);
        (self.sorted, &self.index)
    }


}*/
