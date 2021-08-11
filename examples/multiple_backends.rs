use simple_game_examples::backend;
use backend::{GameLoop, Backend, BackendConf, ImageBackend, MQBackend};

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

// can use any backend and any game loop
fn start_with_backend<T: GameLoop, B: Backend<T> + 'static>(
    conf: BackendConf,
    game_loop: T,
) {
    B::start(conf, game_loop)
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "multiple_backends".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };

    // we use the same game loop and config for two different initializations.
    // first we create an image, then we run it in miniquad:
    start_with_backend::<_, ImageBackend>(my_conf.clone(), my_loop.clone());
    start_with_backend::<_, MQBackend<MyGameLoop>>(my_conf, my_loop);
}
