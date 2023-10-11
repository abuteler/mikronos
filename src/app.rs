use egui::Pos2;

use crate::{timeline::{Timeline, Render}, get_rect};

const PADDING: f32 = 7.0;
const TOOLBAR_HEIGHT: f32 = 25.0;
const HEADER_HEIGHT: f32 = 85.0; // TODO: block of text with customizable data
const SPECTRUM_BG_HEIGHT: f32 = 140.0;
const SPECTRUM_BG_WIDTH: f32 = 580.0;
const HOURS_HEIGHT: f32 = 29.0;
const FOOTER_HEIGHT: f32 = 25.0;

pub const MAX_WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(670.0, {TOOLBAR_HEIGHT+HEADER_HEIGHT+SPECTRUM_BG_HEIGHT+HOURS_HEIGHT+FOOTER_HEIGHT+PADDING*6.0});
pub const MIN_WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(SPECTRUM_BG_WIDTH, {TOOLBAR_HEIGHT+SPECTRUM_BG_HEIGHT+HOURS_HEIGHT});

pub struct App {
    weekday: String,
    compact_mode: bool,
    always_on_top: bool,
    window_decorations: bool,
    timeline: Timeline,
}

impl Default for App {
    fn default() -> Self {
        Self { 
            weekday: "Tuesday".to_string(),
            compact_mode: false,
            always_on_top: false,
            window_decorations: true,
            timeline: Timeline::new(),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // coming soon

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let current_window_size = if self.compact_mode {
            MIN_WINDOW_SIZE
        } else {
            MAX_WINDOW_SIZE
        };

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            //menu bar with icons
            egui::menu::bar(ui, |ui| {
                // left to right: toggle compact mode
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui: &mut egui::Ui| {
                    if ui.add(egui::Button::new({
                        if self.compact_mode {
                            "🔼"
                        } else {
                            "🔽"
                        }
                    })).clicked() {
                        if self.compact_mode {
                            eframe::Frame::set_window_size(frame, MAX_WINDOW_SIZE)
                        } else {
                            eframe::Frame::set_window_size(frame, MIN_WINDOW_SIZE)
                        }
                        self.compact_mode = !self.compact_mode;
                    };
                    ui.label(&self.weekday);
                });
                // right to left: toggle window decoration, toggle always on top
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui: &mut egui::Ui| {
                    if ui.add(egui::Button::new({
                        if self.window_decorations {
                            "🔒"
                        } else {
                            "🔓"
                        }
                    })).clicked() {
                        self.window_decorations = !self.window_decorations;
                        eframe::Frame::set_decorations(frame, self.window_decorations);
                        eframe::Frame::set_window_size(frame, current_window_size) // stick to set size regardless of decorations
                    };
                    if ui.add(egui::Button::new({
                        if self.always_on_top {
                            "⏺"
                        } else {
                            "📌"
                        }
                    })).clicked() {
                        self.always_on_top = !self.always_on_top;
                        eframe::Frame::set_always_on_top(frame, self.always_on_top);
                    };
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            self.timeline.render(ui, get_rect(
                Pos2 { x: 0.0, y: 0.0 },
                Pos2 { x: SPECTRUM_BG_WIDTH, y: SPECTRUM_BG_HEIGHT },
                Pos2 {
                    x: if self.compact_mode { 0.0 } else { PADDING },
                    y: if self.compact_mode { TOOLBAR_HEIGHT } else { TOOLBAR_HEIGHT + HEADER_HEIGHT }
                }
            ));
            ui.add_space(PADDING);
            // TODO: Footer with links and credits.
            ui.label("Footer.")
        });
    }
}
