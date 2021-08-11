use image::{RgbImage};
use super::{Backend, GameLoop, BackendConf};

pub struct ImageBackend{}

impl<T: GameLoop> Backend<T> for ImageBackend {
    fn start(bconf: BackendConf, game_loop: T) where Self: Sized + 'static {
        let mut game_loop = game_loop;
        let bpp = <ImageBackend as Backend<T>>::bytes_per_pixel();
        let image_data = game_loop.init_canvas(bconf.window_width as usize, bconf.window_height as usize, bpp);
        let img_out = RgbImage::from_raw(bconf.window_width as u32, bconf.window_height as u32, image_data).unwrap();
        let output_file_name = format!("{}.png", bconf.window_title);
        img_out.save(output_file_name).unwrap();
    }

    fn bytes_per_pixel() -> usize {
        3
    }
}
