use simple_game_examples::backend;
use backend::{GameLoop, Backend};

pub struct MyGameLoop {

}

impl GameLoop for MyGameLoop {
    fn update(&mut self) {}

    fn draw(&mut self) -> backend::TextureUpdate {
        backend::TextureUpdate::UpdatePart(0, 0, 10, 10, vec![200; 10 * 10 * 4])
    }

    fn init_canvas(&mut self, width: usize, height: usize) -> Vec<u8> {
        vec![0; width * height * 4]
    }
}

fn main() {
    let my_loop = MyGameLoop {};
    backend::MQBackend::start(my_loop);
}
