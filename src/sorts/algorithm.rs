use crate::array::Array;
pub struct Info {
    pub name: String,
    pub description: String,
    pub complexity: Complexity,
    //TODO: it might be easier to just have method return a function that prints the name of the
    //parent directory the file is located in, considering the way I am organizing everything
    pub method: String,
}

pub enum Complexity {
    Constant,
    Logarithmic,
    Linear,
    Linearithmic,
    Quadratic,
    Cubic,
    Exponential,
}

impl Complexity {
    const CONSTANT: (&'static str, &'static str) = ("O(1)", "Constant");
    const LOGARITHMIC: (&'static str, &'static str) = ("O(log n)", "Logarithmic");
    const LINEAR: (&'static str, &'static str) = ("O(n)", "Linear");
    const LINEARITHMIC: (&'static str, &'static str) = ("O(n log n)", "Linearithmic");
    const QUADRATIC: (&'static str, &'static str) = ("O(n^2)", "Quadratic");
    const CUBIC: (&'static str, &'static str) = ("O(n^3)", "Cubic");
    const EXPONENTIAL: (&'static str, &'static str) = ("O(2^n)", "Exponential");

    pub fn get_formula_and_name(&self) -> (&'static str, &'static str) {
        match self {
            Complexity::Constant => Self::CONSTANT,
            Complexity::Logarithmic => Self::LOGARITHMIC,
            Complexity::Linear => Self::LINEAR,
            Complexity::Linearithmic => Self::LINEARITHMIC,
            Complexity::Quadratic => Self::QUADRATIC,
            Complexity::Cubic => Self::CUBIC,
            Complexity::Exponential => Self::EXPONENTIAL,
        }
    }
}

pub trait Algorithm {
    /// Sorts a given [array](crate::array::Array). This method is called in a so
    /// called "algorithm thread".
    fn step(&mut self)-> Vec<u32>;
    /// Returns the name of the algorithm that will be displayed to the user.
    /// Returned value is an owned [String] so it can be generated at runtime.
    fn name(&self) -> String;
    //todo get it to return the info and stuff of the algorithm?
    fn info(&self) -> Info;
}
