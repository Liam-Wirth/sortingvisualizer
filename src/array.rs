use egui::{Color32, Pos2, Shape, Ui, Vec2};
use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

//TODO: Possibly remove this file I dont really think I need it 
#[derive(Clone)]
pub struct Array {
    pub elements: Vec<u32>,
    //element_width: f32,
    //element_gap: f32,
    pub init: bool,
}

impl Array {
    /*
    pub fn new(size: usize, ui: Ui) -> Self {
        let mut rng = thread_rng();
        let mut elements: Vec<Shape> = Vec::new();
        let element_gap = 2.0;
        let element_width = ui.available_width() / size as f32;

        let mut x = 0.0;
        for _ in 0..size {
            let height = rng.gen_range(1..=size) as f32 / size as f32 * ui.available_size().y;
            let rect = egui::Rect::from_min_size(
                Pos2::new(x, ui.available_size().y - height),
                Vec2::new(element_width - 2.0, height),
            );
            let out = Shape::rect_filled(rect, 0.0, Color32::from_rgb(100, 100, 100));
            elements.push(out);
            x += element_width;
        }
        let init = true;
        Array {
            elements,
            element_width,
            element_gap,
            init,
        }
    }
    */
    pub fn new(size: usize) -> Self {
        let mut rng = thread_rng();
        let mut elements: Vec<u32> = Vec::new();
        for i in 1..size {
            elements.push(i as u32);
        }
        elements.shuffle(&mut rng);
        Array {
            elements,
            //element_gap: 2.0,
            //element_width: 2.0,
            init: true,
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.elements.shuffle(&mut rng);
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        self.elements.swap(i, j);
    }
    pub fn get_element(&self, index: usize) -> u32 {
        self.elements[index].clone()
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    pub fn remove(&mut self, index: usize) {
        if index > self.len() {
            return;
        }
        self.elements.remove(index);
    }
    pub fn get_elements(&self) -> Vec<u32> {
        self.elements.clone()
    }
    pub fn set_elements(&mut self, elements: Vec<u32>) {
        self.elements = elements;
    }
    /*
    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().extend(self.elements.clone());
    }
    */
}
