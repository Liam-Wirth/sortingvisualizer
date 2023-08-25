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
    array: Vec<u32>,
    index:  usize,
    sorted: bool,
}

impl BubbleSort {
    pub fn new(mut array: Vec<u32>) -> Self {
        BubbleSort{
            array,
            index: 0,
            sorted: false,
        }
    }
}

impl Algorithm for BubbleSort {
   fn step(&mut self)-> Vec<u32> {
       if self.sorted {
            return self.array.clone()
        }
        let n = self.array.len();
        if self.index >= n - 1 {
            //check to see if it is sorted
            if is_sorted(&self.array) {return self.array.clone()} else { self.index = 0;};
        };
        if self.array.get(self.index) > self.array.get(self.index + 1) {
            self.array.swap(self.index, self.index+1);
        };
        self.index += 1;
        self.array.clone()
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

