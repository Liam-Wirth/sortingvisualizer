use rand::seq::SliceRandom;

pub mod algorithm;
pub mod concurrent;
pub mod distribute;
pub mod exchange;
pub mod hybrid;
pub mod insert;
pub mod merge;
pub mod misc;
pub mod quick;
pub mod select;
pub fn is_sorted(elements: &[u32]) -> bool {
    elements.windows(2).all(|w| w[0] <= w[1])
}
//TODO: Look into writing like a proc macro or something that automatically updates a table with
//all the currently implemented algorithms? for now I'm just gonna hard code it

//NOTE: technically two ways I could handle this, for now I think i'm just gonna
//go the route of just using an enum, cause that's just simpler, and then maybe i'll
//write an impl for the enum that actually pulls stuff from it

pub enum algs {
    BubbleSort,
    StalinSort,
}
