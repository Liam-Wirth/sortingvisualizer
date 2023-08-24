#![allow(unused_imports)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MyApp;
pub mod array;
mod sorts;
use sorts::{
    concurrent,
    distribute,
    exchange,
    hybrid,
    insert,
    merge,
    misc,
    quick,
    select,
    algorithm,
};

