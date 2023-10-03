#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]

pub struct App {
    weekday: String,
    compact_mode: bool
}

impl Default for App {
    fn default() -> Self {
        Self { 
            weekday: "Tuesday".to_string(),
            compact_mode: false,
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            //menu bar with icons
            egui::menu::bar(ui, |ui| {
                let _compact_ui_btn = ui.add(egui::Button::new("−"));
                let _expand_ui_btn = ui.add(egui::Button::new("†"));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.label(&self.weekday);
        });
    }
}