use std::sync::{Arc, Mutex, MutexGuard};

use egui::Color32;

use crate::sorts::algorithm::{self, Algorithm};
#[derive(Debug, Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> Self {
        SharedState(Arc::new(Mutex::new(state)))
    }
    //NOTE: This is key in maintaining data safety amongst shared states in the multithreaded app,
    //essentially ensuring that only one thread can modify the data
    pub fn get(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }
}
///Contains the "state" of the entire [app](crate::app)
#[derive(Debug)]
pub struct State {
    pub time: f64,

    pub speed: f64,

    pub paused: bool,

    pub array: Vec<u32>,

    pub array_writes: u32,
    
    pub current_algorithm: Option<Box<dyn Algorithm>>,
    pub array_reads: u32,
    ///Idealy, this value is never updated manually, and is instead modified by the helper
    ///functions [increment_reads](crate::array::Array::increment_reads()) and
    ///[increment_writes](crate::array::Array::increment_writes())
    pub array_access_count: u32,
    //NOTE: Taken from dmitmetels' sorting visualizer, this is a briliant idea and I wanted to
    //implement it here.
    /// The length of this vector is equal to the [array](State::array)
    /// length, so every color in this vector corresponds to a value with the
    /// exact same index.
    pub colors: Vec<Color32>,
    /// Contains timestamps of the most recent array accesses of every element,
    /// which are drawn like [colored overlays](State::colors). The length of this
    /// vector is equal to the [array](State::array) length, so for every element
    /// in the [array](State::array) _only the most recent_ timestamp is stored
    /// here (not a big deal), so we can avoid memory allocations while program is
    /// running, and this is a pretty simple optimization. To optimize things even
    /// further I've done this: if an element hasn't been accessed yet or it has
    /// been accessed [long ago](crate::app::ACCESSED_VALUE_TIMEOUT), then instead
    /// of using an `Option<f64>` type for every element, I use negative values
    /// because they don't make any sense in the context of time (this saves
    /// [8 bytes](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0e335d0d6109850c7df766fd131c8916)
    /// for every element). When an [algorithm](crate::algorithms::Algorithm)
    /// [reads](crate::array::Array::get) or [writes](crate::array::Array::set) a
    /// value at a certain index from/to the array, the current
    /// [time](State::time) is stored at exactly the same index in this vector.
    pub array_accesses: Vec<f64>,
}
/// A constant that means "there's no array access here". _See_ documentation of
/// [`State.array_accesses`](State::array_accesses) to understand why this
/// constant is a negative number.
pub const NO_ARRAY_ACCESS: f64 = -1.0;
