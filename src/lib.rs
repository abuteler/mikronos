#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod timeline;
pub use app::App;
pub use app::{MIN_WINDOW_SIZE, MAX_WINDOW_SIZE};
pub use timeline::render_timeline;
