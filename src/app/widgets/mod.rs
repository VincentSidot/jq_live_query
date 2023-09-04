pub mod input;
pub mod json;

pub use input::Input;
pub use json::Json;

pub trait Drawable {
    fn draw(
        &self,
        f: &mut ratatui::Frame<impl ratatui::backend::Backend>,
        area: ratatui::layout::Rect,
    );
}
