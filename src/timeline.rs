use eframe::egui;

struct HourComponent {
    time: u8,
    current: bool,
}

struct Timeline {
    time_slots: [HourComponent; 24],
}

// impl Timeline {
//     pub fn new() -> Timeline {
//         let iter = for x in 0..24 {
            
//         }
//     }
// }

pub fn render_timeline (ui: &mut egui::Ui) {
    ui.image(egui::include_image!("../assets/SpectrumBg.png"));
}
    