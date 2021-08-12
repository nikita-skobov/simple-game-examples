use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas};
use backend::{GameLoop, Backend, BackendConf, Event};
use simple_game_examples::{world_screen::WorldScreen, draw::Draw};

#[derive(Default)]
pub struct MyGameLoop {
    pub is_scrolling_up: bool,
    pub is_scrolling_down: bool,
    pub mouse_is_dragging: bool,
    pub mouse_was_clicked: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,

    pub selected_x: f32,
    pub selected_y: f32,

    pub grid_start_x: usize,
    pub grid_start_y: usize,
    pub grid_box_size: usize,
    pub grid_num_boxes: usize,
    canvas: WorldScreen,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self, events: Vec<Event>) {
        self.mouse_was_clicked = false;
        let mut last_pos = None;
        let mut is_scrolling_up = false;
        let mut is_scrolling_down = false;
        for ev in events {
            let (mx, my) = match ev {
                Event::MouseMove { x, y } => (x, y),
                Event::MouseDown { button, x, y } => {
                    if self.mouse_is_dragging { continue; }
                    if let backend::MouseButton::Left = button {
                        self.mouse_is_dragging = true;
                        self.canvas.reset_pan(x, y);
                    }

                    (x, y)
                }
                Event::MouseScroll { up } => {
                    if up {
                        is_scrolling_up = true;
                    } else {
                        is_scrolling_down = true;
                    }
                    continue;
                }
                Event::MouseUp { button, x, y } => {
                    if let backend::MouseButton::Left = button {
                        self.mouse_is_dragging = false;
                    } else {
                        self.mouse_was_clicked = true;
                    }
                    (x, y)
                }
                _ => { continue; }
            };
            last_pos = Some((mx, my));
        }
        self.is_scrolling_down = is_scrolling_down;
        self.is_scrolling_up = is_scrolling_up;
        if let Some(new_pos) = last_pos {
            self.mouse_x = new_pos.0;
            self.mouse_y = new_pos.1;
        }
    }

    fn draw(&mut self) -> backend::TextureUpdate {
        self.canvas.fill(Rgb::WHITE);

        if self.mouse_is_dragging {
            self.canvas.pan_to(self.mouse_x, self.mouse_y);
        }

        let (mouse_after_x, mouse_after_y) = self.canvas.handle_scroll(
            (self.mouse_x, self.mouse_y),
            self.is_scrolling_up, self.is_scrolling_down
        );

        if self.mouse_was_clicked {
            self.selected_x = mouse_after_x;
            self.selected_y = mouse_after_y;
        }

        let box_size = self.grid_box_size;
        let grid_start_x = self.grid_start_x;
        let grid_start_y = self.grid_start_y;
        let num_boxes = self.grid_num_boxes;
        let grid_stop_x = grid_start_x + (num_boxes * box_size);
        let grid_stop_y = grid_start_y + (num_boxes * box_size);

        // draw horizontal lines
        // take account to transform each line position
        // with respect to the correct offsets
        let mut v = grid_start_y as f32;
        for _ in 0..=num_boxes {
            let start_x = grid_start_x as f32;
            let end_x = grid_stop_x as f32;

            self.canvas.draw_horizontal_line_f32(v, start_x, end_x, Rgb::BLACK);

            v += box_size as f32;
        }

        // draw vertical lines
        let mut v = grid_start_x as f32;
        for _ in 0..=num_boxes {
            let start_y = grid_start_y as f32;
            let end_y = grid_stop_y as f32;

            self.canvas.draw_vertical_line_f32(v, start_y, end_y, Rgb::BLACK);

            v += box_size as f32;
        }

        if self.selected_x != 0.0 && self.selected_y != 0.0 {
            let start_x = self.selected_x - 5.0;
            let end_x = self.selected_x + 5.0;
            let start_y = self.selected_y - 5.0;
            let end_y = self.selected_y + 5.0;
            self.canvas.draw_diagonal_line_f32(start_x, start_y, end_x, end_y, Rgb::RED);
        }

        backend::TextureUpdate::UpdateWhole(self.canvas.access_data())
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let color = Rgb::WHITE;
        let new_canvas = Canvas::new_with_color(width, height, color, bpp);
        self.canvas.canvas = new_canvas;
        self.canvas.screen_width = width;
        self.canvas.screen_height = height;
        self.grid_box_size = 20;
        self.grid_start_x = 100;
        self.grid_start_y = 300;
        self.grid_num_boxes = 10;
        self.canvas.scale_x = 1.0;
        self.canvas.scale_y = 1.0;
        self.canvas.scale_factor_up = 1.040;
        self.canvas.scale_factor_down = 0.960;
        let data = self.canvas.access_data();
        data.to_vec()
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "pan_and_zoom_world_screen".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
