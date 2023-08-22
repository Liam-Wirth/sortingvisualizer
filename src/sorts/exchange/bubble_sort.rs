// Import necessary modules
use crate::array::Array;
use crate::sorts::algorithm::{Algorithm, Complexity, Info};

// Define the BubbleSort struct that implements Algorithm trait
pub struct BubbleSort;

impl Algorithm for BubbleSort {
    fn sort(&self, mut array: Array) {
        let n = array.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if array.get_element(j) > array.get_element(j+1){
                    array.swap(j, j + 1);

                    // Add a short delay for visualization purposes
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        }
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
