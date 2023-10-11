#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod timeline;
pub use app::App;
pub use app::{MIN_WINDOW_SIZE, MAX_WINDOW_SIZE};
use egui::{Rect, Pos2};

pub fn get_rect(start: Pos2, end: Pos2, offset: Pos2) -> Rect {
    // because I'm drawing elements absolutely with `ui.put` instead of `ui.add`
    Rect {
        min: Pos2 { x: offset.x + start.x, y: offset.y + start.y },
        max: Pos2 { x: offset.x + end.x, y: offset.y + end.y }
    }
}
