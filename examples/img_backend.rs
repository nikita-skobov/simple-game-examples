use simple_game_examples::backend;
use backend::{GameLoop, Backend, BackendConf};

#[derive(Default, Clone)]
pub struct MyGameLoop {}

impl GameLoop for MyGameLoop {
    fn update(&mut self) {}

    fn draw(&mut self) -> backend::TextureUpdate {
        backend::TextureUpdate::None
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let mut data = vec![0; width * height * bpp];

        // make it purple:
        let mut i = 0;
        while i < data.len() {
            data[i] = 255;
            data[i + 2] = 255;
            i += bpp;
        }
        data
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "hello_world".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::ImageBackend::start(my_conf, my_loop);
}