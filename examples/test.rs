use simple_game_examples::backend;
use backend::{GameLoop, Backend, BackendConf};

#[derive(Default, Clone)]
pub struct MyGameLoop {
    data: Vec<u8>,
    red: u8,
    width: usize,
    height: usize,
    last_i: usize,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self) {}

    fn draw(&mut self) -> backend::TextureUpdate {
        if self.red == 0 {
            self.red = u8::MAX;
        }

        self.data[self.last_i] = self.red;
        self.last_i += 4;
        if self.last_i >= self.data.len() {
            self.last_i = 0;
        }
        // self.red -= 1;
        // self.data = data;
        backend::TextureUpdate::UpdateWhole(&self.data)
    }

    fn init_canvas(&mut self, width: usize, height: usize) -> Vec<u8> {
        let data = vec![0; width * height * 4];
        self.data = data.clone();
        self.red = 255;
        self.width = width;
        self.height = height;
        data
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "hello world!".into(),
        window_width: 400,
        window_height: 800,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
