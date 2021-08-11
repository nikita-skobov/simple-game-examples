use simple_game_examples::backend;
use backend::{GameLoop, Backend, BackendConf, Event};

#[derive(Default, Clone)]
pub struct MyGameLoop {}

impl GameLoop for MyGameLoop {
    fn update(&mut self, events: Vec<Event>) {
        if events.is_empty() { return }
        println!("Events since last update:");
        for event in events {
            println!("{:?}", event);
        }
    }

    fn draw(&mut self) -> backend::TextureUpdate {
        backend::TextureUpdate::None
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        vec![0; width * height * bpp]
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "events".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
