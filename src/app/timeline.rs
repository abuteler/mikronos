use egui::{Ui, Rect, Pos2};
use time::OffsetDateTime;
use crate::{helper::get_rect_with_offset, PADDING};
mod time_cursor;
use time_cursor::Cursor;
mod hour;
use hour::HourComponent;

pub const GAP: f32 = 5.;

enum TimeFormat {
    _Ampm, // TODO
    Continuous,
}

pub trait Render {
    fn render(&mut self, ui: &mut Ui);
}

pub struct Timeline {
    _time_format: TimeFormat,
    time_slots: [HourComponent; 24], //to revise in future https://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
    canvas: Option<Rect>,
    cursor: Cursor,
}

impl Default for Timeline {
    fn default() -> Self {
        let v: Vec<HourComponent> = (0..24).map(HourComponent::new).collect();
        let a: Result<[HourComponent;24], _> = v.try_into();
        Self {
            _time_format: TimeFormat::Continuous,
            time_slots: a.ok().unwrap(),
            canvas: None,
            cursor: Cursor::new(),
        }
    }
}

impl Timeline {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_canvas(&mut self, rect: Rect) {
        self.canvas = Some(rect);
    }
}

impl Render for Timeline {
    fn render(&mut self, ui: &mut Ui) {
        let bg_canvas = self.canvas.unwrap();

        let now = OffsetDateTime::now_local().unwrap().time();
        let current_hour = now.hour();
        let current_mins = now.minute();

        // ui.label(format!("now: {}:{}", current_hour, current_mins));
        // ui.label(format!("variable: {:?}", variable));

        // Display light spectrum BG
        ui.put(bg_canvas,
            egui::Image::new(egui::include_image!("../../assets/SpectrumBg.png"))
        );

        let hour_component_width: f32 = (bg_canvas.max.x-bg_canvas.min.x)/ 24.0 - GAP ;
        let hour_component_height: f32 = 30.0;

        for slot in self.time_slots.as_mut_slice() {
            let val = slot.hour.unwrap();
            let start: f32 = (hour_component_width) * val as f32 ;
            let end: f32 = (hour_component_width) * (val+1) as f32;
            let hour_canvas = get_rect_with_offset(
                Pos2 { x: start, y: 0.0 },
                Pos2 { x: end, y: hour_component_height },
                Pos2 { x: PADDING + GAP*val as f32, y: bg_canvas.max.y }
            );
            slot.set_canvas(hour_canvas);

            // Display hour slots
            ui.put(
                hour_canvas,
                slot.clone()
            );

            if val == current_hour {
                self.cursor.set_canvas(Rect {
                    min: Pos2 { x: hour_canvas.min.x, y: bg_canvas.min.y },
                    max: Pos2 { x: hour_canvas.max.x, y: bg_canvas.max.y },
                });
                self.cursor.minutes = Some(current_mins);
            }
        }

        // Display time cursor
        ui.put(self.cursor.canvas.unwrap(), self.cursor.clone());
    }
}
