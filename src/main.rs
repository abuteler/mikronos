
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(micronos::MAX_WINDOW_SIZE),
        max_window_size: Some(micronos::MAX_WINDOW_SIZE),
        min_window_size: Some(micronos::MIN_WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Micronos",
        native_options,
        Box::new(|cc| Box::new(micronos::App::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(micronos::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
