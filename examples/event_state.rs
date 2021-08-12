use simple_game_examples::backend;
use backend::{GameLoop, Backend, BackendConf, Event, KeyCode};

#[derive(Default, Clone)]
pub struct MyGameLoop {
    pub current_mouse_pos: (f32, f32),
    pub current_held_key: Option<KeyCode>,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::MouseMove { x, y } => {
                    self.current_mouse_pos = (x, y);
                }
                Event::KeyDown { modifier: _, code, repeated: _ } => {
                    self.current_held_key = Some(code);
                }
                Event::KeyUp { modifier: _, code: _ } => {
                    self.current_held_key = None;
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self) -> backend::TextureUpdate {
        println!("Mouse: {:?}, Key: {:?}", self.current_mouse_pos, self.current_held_key);
        backend::TextureUpdate::None
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        vec![0; width * height * bpp]
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "event_state".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
