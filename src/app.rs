pub const MAX_WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(670.0, 420.0);
pub const MIN_WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(650.0, 240.0);

pub struct App {
    weekday: String,
    compact_mode: bool,
    always_on_top: bool,
    window_decorations: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { 
            weekday: "Tuesday".to_string(),
            compact_mode: false,
            always_on_top: false,
            window_decorations: true,
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
                        eframe::Frame::set_window_size(frame, {
                            if self.compact_mode {
                                MIN_WINDOW_SIZE
                            } else {
                                MAX_WINDOW_SIZE
                            }
                        })
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
            ui.image(egui::include_image!("../assets/SpectrumBg.png"));
        });
    }
}