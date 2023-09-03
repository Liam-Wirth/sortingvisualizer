#![allow(unused_imports)]
#![warn(clippy::all, rust_2018_idioms)]

pub use app::MyApp;
pub mod state;
pub mod app;
pub mod array;
mod sorts;
use sorts::{
    algorithm, concurrent, distribute, exchange, hybrid, insert, merge, misc, quick, select,
};
