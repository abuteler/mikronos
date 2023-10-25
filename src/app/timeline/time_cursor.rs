use egui::{Rect, Widget, Ui, Sense, Pos2, Color32, Stroke, Response};

use crate::PADDING;

#[derive(Clone, Default)]
pub struct Cursor {
    pub hour: Option<u8>,
    pub minutes: Option<u8>,
    pub canvas: Option<Rect>,
    width: f32,
    height: f32,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            width: 1.5,
            height: 130.,
            ..Default::default()
        }
    }
    pub fn set_canvas(&mut self, rect: Rect) {
        self.canvas = Some(rect);
    }
}

impl Widget for Cursor {
    fn ui(self, ui: &mut Ui) -> Response {
        let canvas = self.canvas.unwrap();
        let minutes = self.minutes.unwrap();

        let (response, painter) = ui.allocate_painter(
            egui::Vec2 { x: (canvas.max.x - canvas.min.x), y: (canvas.max.y - canvas.min.y) },
            Sense::hover(),
        );

        let canvas_width = canvas.max.x - canvas.min.x;
        let cursor_x: f32 = canvas_width / 60. * minutes as f32 + canvas.min.x;
        let line_start = Pos2 { x: cursor_x, y: canvas.min.y+PADDING };
        let line_end = Pos2 { x: cursor_x, y: canvas.min.y+PADDING + self.height };
        // let line_end = Pos2 { y: canvas.max.y, ..line_start };
        
        painter.line_segment([line_start, line_end], Stroke {
            width: self.width,
            color: Color32::WHITE
        });

        response
    }
}