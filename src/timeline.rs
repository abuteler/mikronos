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
        // consider using ui.add_sized() for fixed widget size.
        ui.image(egui::include_image!("../assets/SpectrumBg.png"));

        ui.add_space(5.0);
        // consider simplifying to ui.horizontal
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui: &mut Ui| {

            for slot in &self.time_slots {
                // probably best if I just implement a Render trait for the HourComponent struct as well.
                // ui.add(); Rect ?
                if slot.current {
                    ui.strong(slot.time.to_string());
                } else {
                    ui.label(slot.time.to_string());
                }
                // padding-right
                ui.add_space(5.0);
            }
        });
        ui.add_space(5.0);
        ui.label("Footer?");
    }
}
    