mod timeline;
use egui::{Pos2, Color32, style};
use timeline::{Timeline, Render};
use crate::helper::get_rect_with_offset;

const PADDING: f32 = 10.0;
const TOOLBAR_HEIGHT: f32 = 40.0;
const HEADER_HEIGHT: f32 = 85.0; // TODO: block of text with customizable data
const SPECTRUM_BG_HEIGHT: f32 = 140.0;
const SPECTRUM_BG_WIDTH: f32 = 580.0;
const HOURS_HEIGHT: f32 = 29.0;
const FOOTER_HEIGHT: f32 = 25.0;

const MAX_WINDOW_HEIGHT: f32 = TOOLBAR_HEIGHT+HEADER_HEIGHT+SPECTRUM_BG_HEIGHT+HOURS_HEIGHT+FOOTER_HEIGHT+PADDING*6.0;
const MIN_WINDOW_HEIGHT: f32 = TOOLBAR_HEIGHT+SPECTRUM_BG_HEIGHT+HOURS_HEIGHT;

pub struct WindowSizes {
    pub max: egui::Vec2,
    pub min: egui::Vec2,
}

pub struct App {
    weekday: String,
    compact_mode: bool,
    always_on_top: bool,
    window_decorations: bool,
    timeline: Timeline,
    pub window_sizes: WindowSizes
}

impl Default for App {
    fn default() -> Self {
        Self { 
            weekday: "Tuesday".to_string(),
            compact_mode: false,
            always_on_top: false,
            window_decorations: true,
            timeline: Timeline::new(),
            window_sizes: WindowSizes {
                max: egui::Vec2::new(770.0, MAX_WINDOW_HEIGHT),
                min: egui::Vec2::new(SPECTRUM_BG_WIDTH, MIN_WINDOW_HEIGHT),
            }
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
            self.window_sizes.min
        } else {
            self.window_sizes.max
        };
        let top_frame = egui::containers::Frame {
            inner_margin: style::Margin::ZERO,
            outer_margin: style::Margin::same(PADDING),
            rounding: egui::Rounding::ZERO,
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::WHITE },
            fill: Color32::BLACK,
            stroke: egui::Stroke::NONE,
        };
        egui::TopBottomPanel::top("top_panel").frame(top_frame).show(ctx, |ui| {
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
                            eframe::Frame::set_window_size(frame, self.window_sizes.max)
                        } else {
                            eframe::Frame::set_window_size(frame, self.window_sizes.min)
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

        let central_frame = egui::containers::Frame {
            inner_margin: style::Margin::ZERO,
            outer_margin: if self.compact_mode {style::Margin::ZERO} else {style::Margin::same(PADDING)},
            rounding: egui::Rounding::ZERO,
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::WHITE },
            fill: Color32::BLACK,
            stroke: egui::Stroke::NONE,
        };
        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui: &mut egui::Ui| {
            let timeline_canvas = get_rect_with_offset(
                Pos2 { x: 0.0, y: 0.0 },
                Pos2 { x: SPECTRUM_BG_WIDTH, y: SPECTRUM_BG_HEIGHT },
                Pos2 {
                    x: if self.compact_mode { 0.0 } else { PADDING },
                    y: if self.compact_mode { TOOLBAR_HEIGHT } else { TOOLBAR_HEIGHT + HEADER_HEIGHT }
                }
            );
            self.timeline.set_canvas(timeline_canvas);
            self.timeline.render(ui);
            ui.add_space(PADDING);
            // TODO: Footer with links and credits.
            ui.label("Footer.")
        });
    }
}
