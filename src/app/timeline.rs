use time::OffsetDateTime;
mod hour;
use egui::{Ui, Rect, Pos2};
use hour::HourComponent;
use crate::{helper::get_rect_with_offset, PADDING};


enum TimeFormat {
    _Ampm, // TODO
    Continuous,
}

pub trait Render {
    fn render (&mut self, ui: &mut Ui, compact: bool);
}

pub struct Timeline {
    time_format: TimeFormat,
    time_slots: [HourComponent; 24], //to revise in future https://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
    canvas: Option<Rect>
}

impl Default for Timeline {
    fn default() -> Self {
        let v: Vec<HourComponent> = (0..24).map(HourComponent::new).collect();
        let a: Result<[HourComponent;24], _> = v.try_into();
        Self {
            time_format: TimeFormat::Continuous,
            time_slots: a.ok().unwrap(),
            canvas: None
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
    fn render (&mut self, ui: &mut Ui, compact: bool) {

        let now = OffsetDateTime::now_local().unwrap().time();
        let current_hour = now.hour();
        let current_mins = now.minute();

        ui.label(format!("now: {}:{}", current_hour, current_mins));

        let canvas = self.canvas.unwrap();
        ui.put(canvas,
            egui::Image::new(egui::include_image!("../../assets/SpectrumBg.png"))
        );

        const GAP: f32 = 1.;
        let hour_component_width: f32 = canvas.max.x/ 24.0 - GAP*2. ;
        let hour_component_height: f32 = 30.0;

        // TODO: implement chronos or similar.
        // Cursor, current, etc ...
        
        // ui.label(format!("time pre: {:?}", self.time_slots[3].time.unwrap()));
        // ui.label(format!("is_none canvas: {:?}", self.canvas.is_none() ));

        for slot in self.time_slots.as_mut_slice() {
            let number = slot.time.unwrap();
            let left_offset: f32 = if compact { 0. } else { PADDING }; 
            let start: f32 = (hour_component_width + GAP) * number as f32 ;
            let end: f32 = (hour_component_width + GAP) * (number+1) as f32;
            let hour_canvas = get_rect_with_offset(
                Pos2 { x: start, y: 0.0 },
                Pos2 { x: end, y: hour_component_height },
                Pos2 { x: left_offset + GAP*number as f32, y: GAP + canvas.max.y }
            );
            slot.set_canvas(hour_canvas);
            ui.put(
                hour_canvas,
                slot.clone()
            );
        }
        // ui.label(format!("time post: {:?}", self.time_slots[3].time.unwrap()));

        // if slot.current {
        //     ui.strong(slot.time.to_string())
        // } else {
        //     ui.label(slot.time.to_string())
        // };
    }
}
