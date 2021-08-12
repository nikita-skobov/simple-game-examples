use crate::draw::Draw;
use crate::Canvas;

#[derive(Default)]
pub struct WorldScreen {
    pub canvas: Canvas,

    pub screen_width: usize,
    pub screen_height: usize,

    pub pan_offset_x: f32,
    pub pan_offset_y: f32,
    pub start_pan_x: f32,
    pub start_pan_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_factor_up: f32,
    pub scale_factor_down: f32,
}

impl WorldScreen {
    // convert from world map space to screen space
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (usize, usize) {
        (
            (self.scale_x * (world_x - self.pan_offset_x)) as usize,
            (self.scale_y * (world_y - self.pan_offset_y)) as usize
        )
    }

    // convert a screen space pixel value to where it is in the world map
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        (
            screen_x / self.scale_x + self.pan_offset_x,
            screen_y / self.scale_y + self.pan_offset_y
        )
    }

    pub fn reset_pan(&mut self, pan_x: f32, pan_y: f32) {
        self.start_pan_x = pan_x;
        self.start_pan_y = pan_y;
    }

    pub fn pan_to(&mut self, pan_x: f32, pan_y: f32) {
        self.pan_offset_x -= (pan_x - self.start_pan_x) / self.scale_x;
        self.pan_offset_y -= (pan_y - self.start_pan_y) / self.scale_y;

        self.start_pan_x = pan_x;
        self.start_pan_y = pan_y;
    }

    /// returns the scroll origin after the scroll calculation
    pub fn handle_scroll(&mut self, scroll_origin: (f32, f32), scrolling_up: bool, scrolling_down: bool) -> (f32, f32) {
        let (mx, my) = scroll_origin;
        let (before_x, before_y) = self.screen_to_world(mx, my);
        if scrolling_up {
            self.scale_x *= self.scale_factor_up;
            self.scale_y *= self.scale_factor_up;
        }
        if scrolling_down {
            self.scale_x *= self.scale_factor_down;
            self.scale_y *= self.scale_factor_down;
        }
        let (after_x, after_y) = self.screen_to_world(mx, my);
        self.pan_offset_x += before_x - after_x;
        self.pan_offset_y += before_y - after_y;

        (after_x, after_y)
    }
}

impl Draw for WorldScreen {
    fn fill(&mut self, color: crate::Rgb) {
        self.canvas.fill(color)
    }

    fn set_pixel_from_index(&mut self, red_index: usize, color: crate::Rgb) {
        self.canvas.set_pixel_from_index(red_index, color)
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: crate::Rgb) {
        self.canvas.set_pixel(x, y, color)
    }

    fn access_data(&self) -> &[u8] {
        self.canvas.access_data()
    }

    fn draw_horizontal_line(&mut self, y: usize, x1: usize, x2: usize, color: crate::Rgb) {
        todo!()
    }

    fn draw_vertical_line(&mut self, x: usize, y1: usize, y2: usize, color: crate::Rgb) {
        todo!()
    }

    fn draw_diagonal_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: crate::Rgb) {
        todo!()
    }

    fn draw_horizontal_line_f32(&mut self, y: f32, x1: f32, x2: f32, color: crate::Rgb) {
        let (start_x, _) = self.world_to_screen(x1, y);
        let (end_x, y_value) = self.world_to_screen(x2, y);
        self.canvas.draw_horizontal_line(y_value, start_x, end_x, color)
    }

    fn draw_vertical_line_f32(&mut self, x: f32, y1: f32, y2: f32, color: crate::Rgb) {
        let (_, start_y) = self.world_to_screen(x, y1);
        let (x_value, end_y) = self.world_to_screen(x, y2);
        self.canvas.draw_vertical_line(x_value, start_y, end_y, color)
    }

    fn draw_diagonal_line_f32(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: crate::Rgb) {
        let (start_x, start_y) = self.world_to_screen(x1, y1);
        let (end_x, end_y) = self.world_to_screen(x2, y2);
        self.canvas.draw_diagonal_line(start_x, start_y, end_x, end_y, color)
    }
}
