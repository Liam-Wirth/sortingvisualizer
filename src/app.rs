//NOTE: LOOK AT THIS!!!!
//https://github.com/dmitmel/sorting-visualization/blob/master/src/app.rs

use std::thread;
use std::time::Duration;

use eframe::egui;
use egui::epaint::shape_transform;
use egui::Vec2;
use egui::{ecolor, Color32, Pos2, Shape, Ui};

use crate::sorts::distribute::bogosort::BogoSort;
use crate::sorts::exchange::bubble_sort::BubbleSort;
use crate::sorts::misc::stalin_sort::StalinSort;
use crate::state::*;
use crate::{
    algorithm::Algorithm,
    array::{self, Array},
};

pub struct MyApp {
    label: String,
    state: SharedState,
    algorithm_thread: thread::JoinHandle<()>,
}
/*
impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "SortingVisualizer".to_owned(),
            sort: None,

        }
    }
}
*/

impl MyApp {
    pub fn init(
        cc: &eframe::CreationContext<'_>,
        algorithm: Option<Box<dyn Algorithm>>,
        array: Vec<u32>,
        label: String,
        speed: f64,
    ) -> Self {
        let array_len = array.len();
        let colors = vec![Color32::TRANSPARENT; array_len];

        let state = SharedState::new(State {
            time: 0.0,
            speed,
            paused: true,
            array,
            array_access_count: 0,
            array_writes: 0,
            array_reads: 0,
            //Algorithm is an option of Box<Dyn> so if it's none, in the future the app will do nothing, right now, it will panic
            current_algorithm: algorithm,
            colors,
            array_accesses: vec![NO_ARRAY_ACCESS; array_len],
        });

        let algorithm_state = state.clone();

        let algorithm_thread = thread::Builder::new()
            .name("algorithm".to_string())
            .spawn(move || {
                //creating the array wrapper and giving it the algorithm state (like a boss)
                let array = Array::new(algorithm_state);
                array.wait(500);
                //NOTE: later I mention being able to dynamically update the chosen sorting algorithm,
                //but I'll worry about that at a later date when I actually set up my ui and stuff, rn
                //the goal is for the app to be able to boot, run a given algorithm, and then stop when
                //the algorithm is done
                //TODO: Right now I am hardcoding the algorithm being used, in the future I would
                //like to learn how to be able to dynamically set this through either shared state,
                //or message passing between threads.
                if algorithm.is_some() {
                    //NOTE: Honestly not sure about the functionality here
                    algorithm.sort(array);
                } else {
                    //HACK: SET THIS TO BE WHATEVER ALGORITHM YOU WANT!! IT WILL BE HARDCODED FOR
                    //NOW!
                    algorithm.sort(array);
                }
            })
            .unwrap();
        Self {
            state,
            algorithm_thread,
            label,
        }
    }
    /*TODO: probably need to re-implement this somehow so that the algorithm can be dynamically
     updated upon like a button being pressed
    pub fn set_sort(&mut self, sort: Box<dyn Algorithm>) {
        self.algorithm = Some(sort);
    }
    */
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);
            //NOTE: this is from the egui discord, I'm pretty sure this operates independant of any other panels, and should control the speed
            //TODO: see if it is possible to implement a slider in another panel that adjusts the speed of this value
            let deltatime = ui.ctx().input(|i| i.stable_dt);
            
            //ctx.request_repaint_after(Duration::from_micros(1));
            ctx.request_repaint();
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
        fn convert_array(
            array: &Vec<u32>,
            ui: &mut Ui,
            index: &usize,
            useColors: bool,
        ) -> Vec<Shape> {
            let mut x = 0.0;
            let element_width = ui.available_width() / array.len() as f32;
            let mut elements: Vec<Shape> = Vec::new();
            for i in array.into_iter() {
                let height = *i as f32 / array.len() as f32 * ui.available_size().y;
                let rect = egui::Rect::from_min_size(
                    Pos2::new(x, ui.available_size().y - height),
                    Vec2::new(element_width, height),
                );
                let mut out = Shape::rect_filled(rect, 0.0, Color32::from_rgb(255, 255, 255));
                if useColors {
                    let hue = (*i as f32 / (array.len() as f32) - 1.0); // map the value to a hue
                    let color = egui::ecolor::Hsva::new(hue, 1.0, 0.5, 1.0);
                    out = Shape::rect_filled(rect, 0.0, color);
                }
                if *i == *index as u32 {
                    if useColors {
                        out = Shape::rect_filled(rect, 0.0, Color32::WHITE)
                    } else {
                        out = Shape::rect_filled(rect, 0.0, Color32::RED)
                    }
                };
                elements.push(out);
                x += element_width;
            }
            elements
        }
    }
}
