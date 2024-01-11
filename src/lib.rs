#![allow(unused_imports)]
#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod state;
pub mod array;
mod sorts;
use sorts::{
    algorithm, concurrent, distribute, exchange, hybrid, insert, merge, misc, quick, select,
};
