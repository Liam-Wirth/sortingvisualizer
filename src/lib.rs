#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MyApp;
pub mod array;
```rust
pub struct Array {
    elements: Vec<Shape>,
    element_width: f32,
    element_gap: f32,
    init: bool,
}

impl Array {
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
//Other methods such as shuffle, is_sorted, swap, get_element, and remove, are also present, but I didnt paste them here because I don't feel they are relevant
pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().extend(self.elements.clone());
    }
}
```
