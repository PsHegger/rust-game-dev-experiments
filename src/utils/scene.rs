use piston_window::{Context, G2d};

pub trait Scene {
    fn render(self, c: Context, g: &mut G2d);
    fn update(&mut self);
    fn on_resize(&mut self, new_width: u32, new_height: u32);
}
