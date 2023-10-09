use eframe::egui;
use egui::Ui;

struct HourComponent {
    time: u8,
    current: bool,
}
enum TimeFormat {
    AMPM,
    CONTINUOUS,
}
pub trait Render {
    fn render (&self, ui: &mut Ui) -> ();
}
pub struct Timeline {
    time_format: TimeFormat,
    // time_slots: [HourComponent; 24], to revise in future https://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
    time_slots: Vec<HourComponent>,
}

impl Timeline {
    pub fn new() -> Self {
        let iter = (0..24).map(|index| HourComponent {
            time: index,
            current: false
        });
        return  Self {
            time_format: TimeFormat::CONTINUOUS,
            time_slots: Vec::from_iter(iter)
        }
    }
}

impl Render for Timeline {
    fn render (&self, ui: &mut Ui) {
        ui.image(egui::include_image!("../assets/SpectrumBg.png"));
    }
}
    