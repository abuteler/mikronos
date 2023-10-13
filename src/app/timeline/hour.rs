use egui::{Widget, Rect, Ui, Response, Sense, Pos2, Stroke, Color32};

#[derive(Clone, Default)]
pub struct HourComponent {
    pub time: Option<u8>,
    _current: bool, // TODO
    canvas: Option<Rect>
}

impl HourComponent {
    pub fn new(time:u8) -> Self {
        Self {
            time: Some(time),
            ..Default::default()
        }
    }
    pub fn set_canvas(&mut self, rect: Rect) {
        self.canvas = Some(rect);
    }
}

impl Widget for HourComponent {
    fn ui (self, ui: &mut Ui) -> Response {
        let canvas: Rect = self.canvas.unwrap();
        let label: String = format!("{:02}", self.time.unwrap());
        let (response, painter) = ui.allocate_painter(
                egui::Vec2 { x: (canvas.max.x - canvas.min.x), y: (canvas.max.y - canvas.min.y) },
                Sense::hover()
            );
        // let painter = ui.painter();
        let line_start = Pos2 { x: canvas.min.x, y: canvas.min.y };
        let line_end = Pos2 { x: canvas.max.x, y: canvas.min.y };
        painter.line_segment([line_start, line_end], Stroke {
            width: 2.0,
            color: Color32::WHITE
        });
        // painter.add(Shape::text(canvas.center(), pos, Align2::CENTER_CENTER, text, font_id, color));
        
        /*
        pub struct Response {
            pub ctx: Context,
            pub layer_id: LayerId,
            pub id: Id,
            pub rect: Rect,
            pub sense: Sense,
            /* private fields */
        }
        */
        response
    }
}
