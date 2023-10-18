use egui::{Widget, Rect, Ui, Response, Sense, Pos2, Stroke, Color32, Align2, FontId};


#[derive(Clone, Default)]
pub struct HourComponent {
    pub hour: Option<u8>,
    _current: bool, // TODO
    canvas: Option<Rect>
}

impl HourComponent {
    pub fn new(hour:u8) -> Self {
        Self {
            hour: Some(hour),
            ..Default::default()
        }
    }
    pub fn set_canvas(&mut self, rect: Rect) {
        self.canvas = Some(rect);
    }
}

impl Widget for HourComponent {
    fn ui(self, ui: &mut Ui) -> Response {
        let canvas: Rect = self.canvas.unwrap();
        let hour = self.hour.unwrap();
        let label: String = format!("{:02}", hour);
        let (response, painter) = ui.allocate_painter(
                egui::Vec2 { x: (canvas.max.x - canvas.min.x), y: (canvas.max.y - canvas.min.y) },
                Sense::hover()
            );
        let line_start = Pos2 { x: canvas.min.x, y: canvas.min.y };
        let line_end = Pos2 { x: canvas.max.x, y: canvas.min.y };
        painter.line_segment([line_start, line_end], Stroke {
            width: 3.0,
            color: Color32::WHITE
        });
        let horizontal_offset = (line_end.x - line_start.x)/2.;
        painter.text(Pos2 { x: line_start.x + horizontal_offset, y: line_start.y+8. }, Align2::CENTER_TOP, label, FontId::proportional(16.), Color32::WHITE);
        
        // if slot.current {
        //     ui.strong(slot.hour.to_string())
        // } else {
        //     ui.label(slot.hour.to_string())
        // };
        response
    }
}
