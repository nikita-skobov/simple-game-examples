use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas};
use backend::{GameLoop, Backend, BackendConf, Event};
use simple_game_examples::draw::Draw;

#[derive(Default)]
pub struct MyGameLoop {
    canvas: Canvas,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self, _events: Vec<Event>) {}

    fn draw(&mut self) -> backend::TextureUpdate {
        backend::TextureUpdate::None
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let color = Rgb::WHITE;
        let mut new_canvas = Canvas::new_with_color(width, height, color, bpp);
        let box_size = 20;
        let mut i = box_size;
        while i < height {
            new_canvas.draw_horizontal_line(i, 0, width, Rgb::BLACK);
            i += box_size;
        }
        i = box_size;
        while i < width {
            new_canvas.draw_vertical_line(i, 0, height, Rgb::BLACK);
            i += box_size;
        }

        self.canvas = new_canvas;
        let data = self.canvas.access_data();
        data.to_vec()
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "simple_grid".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::ImageBackend::start(my_conf, my_loop);
}
