use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas};
use backend::{GameLoop, Backend, BackendConf, Event};
use simple_game_examples::draw::Draw;

pub struct Tile {
    pub color: Rgb,
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub pt3: (usize, usize),
    pub pt4: (usize, usize),
    pub width: usize,
    pub height: usize,
}

impl Tile {
    pub fn square(size: usize) -> Tile {
        let width = size;
        let height = size;
        Tile::new(width, height)
    }

    pub fn new(width: usize, height: usize) -> Tile {
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
}

pub struct GameMap {
    tile: Tile,
    pub map_size: (usize, usize),
    pub map_origin: (usize, usize),
}

impl GameMap {
    pub fn transform_coordinate(&self, x: usize, y: usize) -> (usize, usize) {
        let tx = x as isize;
        let ty = y as isize;
        let new_x = (self.map_origin.0 * self.tile.width) as isize + (tx - ty) * (self.tile.width / 2) as isize;
        let new_y = (self.map_origin.1 * self.tile.height) as isize + (tx + ty) * (self.tile.height / 2) as isize;
        (new_x as usize, new_y as usize)
    }

    pub fn draw_tile(&self, x: usize, y: usize, canvas: &mut Canvas) {
        let (new_x, new_y) = self.transform_coordinate(x, y);
        let tile = &self.tile;
        let pt1 = (tile.pt1.0 + new_x, tile.pt1.1 + new_y);
        let pt2 = (tile.pt2.0 + new_x, tile.pt2.1 + new_y);
        let pt3 = (tile.pt3.0 + new_x, tile.pt3.1 + new_y);
        let pt4 = (tile.pt4.0 + new_x, tile.pt4.1 + new_y);
        canvas.draw_diagonal_line(pt1.0, pt1.1, pt2.0, pt2.1, tile.color);
        canvas.draw_diagonal_line(pt2.0, pt2.1, pt3.0, pt3.1, tile.color);
        canvas.draw_diagonal_line(pt3.0, pt3.1, pt4.0, pt4.1, tile.color);
        canvas.draw_diagonal_line(pt4.0, pt4.1, pt1.0, pt1.1, tile.color);
    }
}

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

        let mut tile = Tile::new(40, 20);
        tile.color = Rgb::BLUE;
        let game_map = GameMap {
            tile,
            map_size: (20, 10),
            map_origin: (5, 1),
        };
        for y in 0..game_map.map_size.1 {
            for x in 0..game_map.map_size.0 {
                game_map.draw_tile(x, y, &mut new_canvas);
            }
        }

        self.canvas = new_canvas;
        let data = self.canvas.access_data();
        data.to_vec()
    }
}

fn main() {
    let my_loop = MyGameLoop::default();
    let my_conf = BackendConf {
        window_title: "tiled_map".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::ImageBackend::start(my_conf, my_loop);
}
