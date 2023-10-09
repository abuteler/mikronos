#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod timeline;
pub use app::App;
pub use app::{MIN_WINDOW_SIZE, MAX_WINDOW_SIZE};
