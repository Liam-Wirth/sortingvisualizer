use egui::{Ui, Pos2, Color32, Shape};
use eframe::{egui};
use egui::Vec2;

use crate::{array::{self, Array}};

pub struct MyApp {
    label: String,
    array: Array,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "SortingVisualizer".to_owned(),
            array: Array::new(20),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);
            if self.array.init {
                let draw = convert_array( self.array.get_elements(), ui);
                ui.painter().extend(draw.clone());
                self.array.shuffle();
            }
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
            
        }
        fn convert_array(array: Vec<u32>, ui: &mut Ui) -> Vec<Shape> {
            let mut x = 0.0;
            let element_width = ui.available_width()/array.len() as f32;
            let mut elements: Vec<Shape> = Vec::new();
            for i in &array {
               let height = *i as f32 / array.len() as f32 * ui.available_size().y;
                let rect = egui::Rect::from_min_size(
                   Pos2::new(x, ui.available_size().y - height),
                    Vec2::new(element_width-2.0, height),
                );
                let out = Shape::rect_filled(rect, 0.0, Color32::from_rgb(100, 100, 100));
                elements.push(out);
                x += element_width;
            }; 
            elements
        }
    }
}
