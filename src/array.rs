use egui::plot::BarChart;
use egui::widgets::plot::Bar;
use egui::{Color32, Pos2, Shape, Ui, Vec2};
use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serde::de::SeqAccess;
//NOTE: honestly, might be better to just refactor the array struct to be an integer of egui Bars
pub struct Array {
    elements: Vec<Shape>,
    element_width: f32,
    element_gap: f32,
}

impl Array {
    pub fn new(size: usize,  available_width: f32, available_size: Vec2) -> Self {
        let mut rng = thread_rng();
        let mut elements: Vec<Shape> = Vec::new();
        let element_gap = 2.0;
        let element_width = available_width / size as f32;
        let mut x = 0.0;
        for _ in 0..size {
            let height = rng.gen_range(1..=size) as f32 / size as f32 * available_size.y;
            let rect = egui::Rect::from_min_size(
                Pos2::new(x, available_size.y - height),
                Vec2::new(element_width - 2.0, height),
            );
            let out = Shape::rect_filled(rect, 0.0, Color32::from_rgb(100,100,100));
            elements.push(out);
            x += element_width;
        }
        Array {
            elements,
            element_width,
            element_gap,
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.elements.shuffle(&mut rng);
    }
    pub fn is_sorted(&self) -> bool {
        for i in 0..self.elements.len() - 1 {
            //NOTE: this is likely unsafe code LOLE!
            let (rect1 ,rect2) = (&self.elements[i], &self.elements[i+1]);
               if rect1.visual_bounding_rect().height()> rect2.visual_bounding_rect().height() {
                return false;
               } 
            }
        true
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        self.elements.swap(i, j);
    }
    pub fn get_element(&self, index: usize) -> Shape {
        self.elements[index].clone()
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    pub fn remove(&mut self, index: usize) {
        if index>self.len() {
            return
        }
        self.elements.remove(index);
    }
    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().extend(self.elements.clone());
    }
    
}
