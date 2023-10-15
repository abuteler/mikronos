#![warn(clippy::all, rust_2018_idioms)]

mod app;    
pub mod helper;

pub use app::App;

pub const PADDING: f32 = 10.;