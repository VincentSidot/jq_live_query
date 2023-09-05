pub mod input;
pub mod json;
pub mod jq;

use std::fmt::Error;

use ratatui::backend::Backend;

pub use input::Input;
pub use json::Json;
pub use jq::Jq;

pub trait Drawable {
    fn draw<B: Backend>(
        &self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) -> Result<(), Error>;
}
