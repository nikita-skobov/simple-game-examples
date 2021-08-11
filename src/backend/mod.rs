pub mod b_miniquad;
pub use b_miniquad::*;

pub mod b_image;
pub use b_image::*;

/// copied from miniquad Conf and modified to remove
/// parts that arent relevant to us.
#[derive(Debug, Clone)]
pub struct BackendConf {
    /// Title of the window, defaults to an empty string.
    pub window_title: String,
    /// The preferred width of the window, ignored on wasm/android.
    ///
    /// Default: 800
    pub window_width: i32,
    /// The preferred height of the window, ignored on wasm/android.
    ///
    /// Default: 600
    pub window_height: i32,
    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fullscreen: bool,
    /// Determines if the application user can resize the window
    pub window_resizable: bool,
}

impl Default for BackendConf {
    fn default() -> BackendConf {
        BackendConf {
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            fullscreen: false,
            window_resizable: true,
        }
    }
}


pub trait GameLoop {
    fn update(&mut self);
    fn draw(&mut self) -> TextureUpdate;
    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8>;
}

pub trait Backend<T: GameLoop> {
    fn start(_conf: BackendConf, _game_loop: T) where Self: Sized + 'static {}
    fn bytes_per_pixel() -> usize;
}

pub enum TextureUpdate<'a> {
    None,
    UpdateWhole(&'a [u8]),
    UpdatePart(i32, i32, i32, i32, Vec<u8>),
}
