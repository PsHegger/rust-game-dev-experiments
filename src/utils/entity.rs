use piston_window::{Context, G2d};

pub trait Renderable {
    fn render(self, c: Context, g: &mut G2d);
}

pub trait Updatable {
    type Args;

    fn update(&mut self, args: Self::Args);
}
