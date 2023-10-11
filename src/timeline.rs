use eframe::egui_glow::{painter, Painter};
use egui::{Ui, Rect, Pos2};

use crate::get_rect;

struct HourComponent {
    time: u8,
    current: bool,
}

enum TimeFormat {
    AMPM,
    CONTINUOUS,
}

pub trait Render {
    fn render (&self, ui: &mut Ui, rect: Rect);
}

pub struct Timeline {
    time_format: TimeFormat,
    // time_slots: [HourComponent; 24], to revise in future https://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
    time_slots: Vec<HourComponent>,
}

impl Default for Timeline {
    fn default() -> Self {
        let iter = (0..24).map(|index| HourComponent {
            time: index,
            current: false
        });

        Self {
            time_format: TimeFormat::CONTINUOUS,
            time_slots: Vec::from_iter(iter)
        }
    }
}

impl Timeline {
    pub fn new() -> Self {
        Default::default()
    }
}

// impl Render for HourComponent {
//     fn render (&self, ui: &mut Ui, rect: Rect) {
//         let (response, painter) = ui.allocate_painter(egui::Vec2 { x: (rect.max.x - rect.min.x), y: (rect.max.y - rect.min.y) }, sense);
//     }
// }

impl Render for Timeline {
    fn render (&self, ui: &mut Ui, rect: Rect) {
        const PADDING: f32 = 0.2;
        ui.put(rect,
            egui::Image::new(egui::include_image!("../assets/SpectrumBg.png"))
        );

        let hour_component_width: f32 = (rect.max.x - PADDING * 22.0) / 24.0;
        let hour_component_height: f32 = 29.0;

        // TODO: implement chronos or similar.
        // Cursor, current, etc ...

        for (i, slot) in self.time_slots.iter().enumerate() {
            // probably best if I just implement a Render trait for the HourComponent struct as well.
            let label: String = format!("{:02}", slot.time);
            let start: f32 = (hour_component_width + PADDING) * i as f32 ;
            let end: f32 = (hour_component_width + PADDING) * (i+1) as f32;

            ui.put(
                get_rect(
                    Pos2 { x: start, y: 0.0 },
                    Pos2 { x: end, y: hour_component_height },
                    Pos2 { x: PADDING*i as f32, y: PADDING + rect.max.y }
                ),
                egui::Label::new(label) // I should probably implement HourComponent as a Widget and normalize my solution towards egui.
            );

            // if slot.current {
            //     ui.strong(slot.time.to_string())
            // } else {
            //     ui.label(slot.time.to_string())
            // };
            // ui.label(format!("rect.max.x {:}, hour width {}", rect.max.x, hour_component_width));
        }
    }
}
