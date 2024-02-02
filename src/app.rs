use std::time::Duration;

use eframe::egui;
use egui::Vec2;
use egui::{Color32, Pos2, Shape, Ui};

use crate::sorts::distribute::bogosort::BogoSort;
use crate::{
    array::{self, Array},
    sorts::{algorithm::Algorithm, exchange::bubble_sort::BubbleSort},
};

pub struct MyApp {
    label: String,
    array: Array,
    //CREDIT: thanks to phind.com ai for teaching me about this.
    //NOTE:
    //Option: Indicates wheter or not a sorting algorithm is currently assigned (allows some/none)
    //Box: Type is used to allocate it to the heap allowing for dynamic dispatch.
    //dyn Algorithm indicates that the type being stored is an object with the Algorithm trait
    //
    //NOTE: The reason I do this is to provide flexibility in storing the state of the current
    //algorithms iteration, and easy switching to any different algorithm I use
    sort: Option<Box<dyn Algorithm>>,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "SortingVisualizer".to_owned(),
            array: Array::new(1000),
            sort: None,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
    pub fn set_sort(&mut self, sort: Box<dyn Algorithm>) {
        self.sort = Some(sort);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| { 
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);
            //NOTE: this is from the egui discord, I'm pretty sure this operates independant of any other panels, and should control the speed
            //TODO: see if it is possible to implement a slider in another panel that adjusts the speed of this value
            let deltatime = ui.ctx().input(|i| i.stable_dt);
            if self.array.init {
                if self.sort.is_none() {
                    //HACK: clone so that the elements dont get consumed cause otherwise I'd use a
                    //borrow but then i'd need to set lifetimes and that is confusing
                    //NOTE: jesus christ wtf was I smoking
                    self.set_sort(Box::new(BubbleSort::new(self.array.len())));
                }
                let sorted:(bool,&usize)= if let Some(sort) = self.sort.as_mut() {
                    sort.step(&mut self.array.elements)
                } else {
                    (false, &0)
                };
                if sorted.0 {
                    //TODO: implement that fun final animation for the algorithm that shows the thingy running through the full array, and then flashing it as green    
                };
                let draw = convert_array(&self.array.elements, ui, sorted.1);
                ui.painter().extend(draw.clone());
                
            }
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
        fn convert_array(array: &Vec<u32>, ui: &mut Ui, index: &usize) -> Vec<Shape> {
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
                if *i == *index as u32{
                    out = Shape::rect_filled(rect, 0.0, Color32::from_rgb(255, 0, 0));
                };
                elements.push(out);
                x += element_width;
            }
            elements
        }
    }
}
