use egui::Color32;
use rand::seq::SliceRandom;
//NOTE: this is the thing that I edit if I am ever working on my shit and realize "ruh roh, I need
//to be able to do this thing, but cant currently do this thing"

use crate::state::*;
#[derive(Debug, Clone)]
pub struct Array(SharedState);

impl Array {
    pub fn new(state: SharedState) -> Self {
        Array(state)
    }
    ///Puts the current thread to sleep for specified amount of time, and blocks the thread if the
    ///animation is paused
    pub fn wait(&self, ms: u64) {
        use std::thread;
        use std::time::Duration;
        // state must be locked for as short as possible so we shouldn't keep it
        // locked while sleeping (`thread::sleep` and `thread::park`)
        thread::sleep(Duration::from_micros({
            let state = self.0.get(); //NOTE: this is locking the shared state and giving ownership
                                      //of it to this variable
            (ms as f64 * 1000.0 / state.speed) as u64
        }));
        let paused = {
            let state = self.0.get();
            state.paused
        };

        if paused {
            thread::park();
        }
    }
    //NOTE: This is the beginning of more API type stuff :)
    /// Returns the length of the underlying vector in state
    pub fn len(&self) -> usize {
        let state = self.0.get();
        state.array.len()
    }
    pub fn get(&self, index: usize) -> u32 {
        let mut state = self.0.get();
        let value = state.array[index];
        //Incrementing reads
        self.increment_reads();
        state.array_accesses[index] = state.time;
        value
    }

    /// Sets a value of the at a given index.
    pub fn set(&self, index: usize, value: u32) {
        let mut state = self.0.get();
        state.array[index] = value;
        self.increment_writes();
        state.array_accesses[index] = state.time;
    }
    pub fn swap(&self, a: usize, b: usize) {
        let mut state = self.0.get();
        state.array.swap(a, b);
        self.increment_writes();
    }
    //TODO: Refactor this so that its more of a procedural thing that could be animated?
    pub fn shuffle(&self) {
        use rand::{thread_rng, Rng};
        let mut state = self.0.get();
        let mut rng = rand::thread_rng();
        state.array.shuffle(&mut rng);
    }
    //Removes an element at the given index, and returns the removed element to the user
    pub fn remove_element(&self, index: usize) -> u32 {
        let mut state = self.0.get();
        state.array.remove(index)
    }

    pub fn increment_reads(&self) {
        let mut state = self.0.get();
        state.array_reads += 1;
        state.array_access_count += 1;
    }

    pub fn increment_writes(&self) {
        let mut state = self.0.get();
        state.array_writes += 1;
        state.array_access_count += 1;
    }
    /// Resets color of the value at a given index (sets it to the transparent
    /// color).
    ///
    /// _See_ [`State.colors`](crate::state::State::colors)
    pub fn reset_color(&self, index: usize) {
        self.set_color(index, Color32::TRANSPARENT);
    }

    /// Sets color of the value at a given index.
    ///
    /// _See_ [`State.colors`](crate::state::State::colors)
    pub fn set_color(&self, index: usize, color: Color32) {
        let mut state = self.0.get();
        state.colors[index] = color;
    }
    pub fn regen_array(&self, len: usize){
        let mut state = self.0.get();
        //It's pointless IMO to have the array visualize from 0..len cause 0 could never be drawn
        state.array = vec![1..len];
        state.array_accesses = vec![NO_ARRAY_ACCESS; len];
        state.array_reads = 0;
        state.array_writes = 0;
        state.array_accesses = 0;
    }
}
