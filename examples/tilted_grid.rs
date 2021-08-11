use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas};
use backend::{GameLoop, Backend, BackendConf};

pub struct Tile {
    pub color: Rgb,
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub pt3: (usize, usize),
    pub pt4: (usize, usize),
    pub width: usize,
    pub height: usize,
}

pub fn square_tile(size: usize) -> Tile {
    let width = size;
    let height = size;
    Tile {
        color: Rgb::BLACK,
        pt1: (width / 2, 0),
        pt2: (width, height / 2),
        pt3: (width / 2, height),
        pt4: (0, height / 2),
        width,
        height,
    }
}

#[derive(Default)]
pub struct MyGameLoop {
    canvas: Canvas,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self) {}

    fn draw(&mut self) -> backend::TextureUpdate {
        backend::TextureUpdate::None
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let color = Rgb::WHITE;
        let mut new_canvas = Canvas::new_with_color(width, height, color, bpp);

        let tile = square_tile(20);
        let num_tiles_in_row = width / tile.width;
        let num_tiles_in_col = height / tile.height;
        let mut tile_offset_x = 0;
        let mut tile_offset_y = 0;
        for _ in 0..num_tiles_in_col {
            for _ in 0..num_tiles_in_row {
                // pt1 to pt2:
                let pt1 = (tile.pt1.0 + tile_offset_x, tile.pt1.1 + tile_offset_y);
                let pt2 = (tile.pt2.0 + tile_offset_x, tile.pt2.1 + tile_offset_y);
                // println!("Drawing from {:?} to {:?}", pt1, pt2);
                new_canvas.draw_diagonal_line(pt1.0, pt1.1, pt2.0, pt2.1, tile.color);

                // pt2 to pt3
                let pt3 = (tile.pt3.0 + tile_offset_x, tile.pt3.1 + tile_offset_y);
                // println!("Drawing from {:?} to {:?}", pt2, pt3);
                new_canvas.draw_diagonal_line(pt2.0, pt2.1, pt3.0, pt3.1, tile.color);
                // break;

                // pt3 to pt4
                let pt4 = (tile.pt4.0 + tile_offset_x, tile.pt4.1 + tile_offset_y);
                new_canvas.draw_diagonal_line(pt3.0, pt3.1, pt4.0, pt4.1, tile.color);

                // pt4 to pt1
                new_canvas.draw_diagonal_line(pt4.0, pt4.1, pt1.0, pt1.1, tile.color);

                tile_offset_x += tile.width;
            }
            // break;
            tile_offset_x = 0;
            tile_offset_y += tile.height;
        }
        

        self.canvas = new_canvas;
        let data = self.canvas.access_data();
        data.to_vec()
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "tilted_grid".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::ImageBackend::start(my_conf, my_loop);
}
