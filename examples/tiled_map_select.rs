use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas, Point, LineSegment};
use backend::{GameLoop, Backend, BackendConf, Event};
use simple_game_examples::draw::Draw;

#[derive(Debug, Default)]
pub struct Tile {
    pub color: Rgb,
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub pt3: (usize, usize),
    pub pt4: (usize, usize),
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub enum TileCorner {
    NotInCorner,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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

    pub fn get_line_segments(&self) -> [LineSegment; 4] {
        let pt1 = Point::from(self.pt1);
        let pt2 = Point::from(self.pt2);
        let pt3 = Point::from(self.pt3);
        let pt4 = Point::from(self.pt4);

        let ls1 = LineSegment::from((pt1, pt2));
        let ls2 = LineSegment::from((pt2, pt3));
        let ls3 = LineSegment::from((pt3, pt4));
        let ls4 = LineSegment::from((pt4, pt1));

        [ls1, ls2, ls3, ls4]
    }

    pub fn point_is_outside_tile(&self, point: Point) -> TileCorner {
        let ls = LineSegment {
            p1: point,
            p2: Point {
                x: self.width / 2,
                y: self.height / 2,
            }
        };

        // the point is outside the tile if the line
        // segment between the point and the
        // center of the tile intersects
        // any of the 4 line segments of this tile:

        let [ls1, ls2, ls3, ls4] = self.get_line_segments();
        if ls1.intersects(ls) {
            return TileCorner::TopRight;
        }
        if ls2.intersects(ls) {
            return TileCorner::BottomRight;
        }
        if ls3.intersects(ls) {
            return TileCorner::BottomLeft;
        }
        if ls4.intersects(ls) {
            return TileCorner::TopLeft;
        }

        TileCorner::NotInCorner
    }
}

#[derive(Debug, Default)]
pub struct GameMap {
    tile: Tile,
    pub map_size: (usize, usize),
    pub map_origin: (usize, usize),
    pub shift_x: isize,
    pub shift_y: isize,
    pub map_to_screen_transform: [isize; 4],
}

impl GameMap {
    pub fn calculate_transform(&mut self) {
        let shift_x = (self.map_origin.0 * self.tile.width) as isize;
        let shift_y = (self.map_origin.1 * self.tile.height) as isize;
        self.shift_x = shift_x;
        self.shift_y = shift_y;

        // matrix multiplication. 2d matrix:
        // |tx| * [ A B ]
        // |ty| * [ C D ]
        let a = (self.tile.width / 2) as isize;
        let b = -a;
        let c = (self.tile.height / 2) as isize;
        let d = c;
        self.map_to_screen_transform = [a, b, c, d];
    }

    pub fn transform_coordinate(&self, x: usize, y: usize) -> (usize, usize) {
        let tx = x as isize;
        let ty = y as isize;
        let shift_x = self.shift_x;
        let shift_y = self.shift_y;

        let [a, b, c, d] = self.map_to_screen_transform;
        let new_x = (tx * a) + (ty * b);
        let new_y = (tx * c) + (ty * d);
        // shift is a lateral transformation.
        let new_x = shift_x + new_x;
        let new_y = shift_y + new_y;
        (new_x as usize, new_y as usize)
    }

    pub fn mouse_to_world_coordinate(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let (cell_x, cell_y) = (x as usize / self.tile.width, y as usize / self.tile.height);

        let selected = (
            (cell_y as isize - self.map_origin.1 as isize) + (cell_x as isize - self.map_origin.0 as isize),
            (cell_y as isize - self.map_origin.1 as isize) - (cell_x as isize - self.map_origin.0 as isize)
        );

        let within_tile = (x as usize % self.tile.width, y as usize % self.tile.height);
        let within_tile = Point::from(within_tile);
        let point_tile_position = self.tile.point_is_outside_tile(within_tile);

        let (selected_cell_x, selected_cell_y) = match &point_tile_position {
            TileCorner::NotInCorner => (selected.0, selected.1),
            TileCorner::TopLeft => (selected.0 - 1, selected.1),
            TileCorner::TopRight => (selected.0, selected.1 - 1),
            TileCorner::BottomLeft => (selected.0, selected.1 + 1),
            TileCorner::BottomRight => (selected.0 + 1, selected.1),
        };
        if selected_cell_x < 0 || selected_cell_y < 0 {
            return None;
        }
        let (selected_cell_x, selected_cell_y) = (selected_cell_x as usize, selected_cell_y as usize);
        if selected_cell_x >= self.map_size.0 || selected_cell_y >= self.map_size.1 {
            return None;
        }
        Some((selected_cell_x, selected_cell_y))
    }

    pub fn draw_tile(&self, x: usize, y: usize, canvas: &mut Canvas) {
        let (new_x, new_y) = self.transform_coordinate(x, y);
        // println!("Drawing {}, {} at {:?}", x, y, (new_x, new_y));
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
    map: GameMap,
    mouse_position: (f32, f32),
}

impl GameLoop for MyGameLoop {
    fn update(&mut self, events: Vec<Event>) {
        let mut last_pos = None;
        for ev in events {
            let (mx, my) = match ev {
                Event::MouseMove { x, y } => (x, y),
                _ => { continue; }
            };
            last_pos = Some((mx, my));
        }
        if let Some(new_pos) = last_pos {
            self.mouse_position = new_pos;
        }
    }

    fn draw(&mut self) -> backend::TextureUpdate {
        // always re draw the tiles:
        self.canvas.fill(Rgb::WHITE);
        for y in 0..self.map.map_size.1 {
            for x in 0..self.map.map_size.0 {
                self.map.draw_tile(x, y, &mut self.canvas);
            }
        }

        // calculate the mouse position to world map position:
        // if its outside of the map, then dont draw any highlight
        // and just return the blank grid:
        let (x, y) = self.mouse_position;
        let (selected_cell_x, selected_cell_y) = match self.map.mouse_to_world_coordinate(x, y) {
            Some(o) => o,
            None => {
                return backend::TextureUpdate::UpdateWhole(self.canvas.access_data());
            }
        };

        // otherwise its within bounds, so transform the
        // world coordinate back to screen coordinate, and then draw
        // a box of the tile size around the users mouse:
        let screen_coord = self.map.transform_coordinate(selected_cell_x as usize, selected_cell_y as usize);
        let (cell_start_x, cell_start_y) = screen_coord;
        let cell_end_x = cell_start_x + self.map.tile.width;
        let cell_end_y = cell_start_y + self.map.tile.height;
        self.canvas.draw_horizontal_line(cell_start_y, cell_start_x, cell_end_x, Rgb::RED);
        self.canvas.draw_horizontal_line(cell_end_y, cell_start_x, cell_end_x, Rgb::RED);
        self.canvas.draw_vertical_line(cell_start_x, cell_start_y, cell_end_y, Rgb::RED);
        self.canvas.draw_vertical_line(cell_end_x, cell_start_y, cell_end_y, Rgb::RED);
        backend::TextureUpdate::UpdateWhole(self.canvas.access_data())
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let color = Rgb::WHITE;
        let mut new_canvas = Canvas::new_with_color(width, height, color, bpp);

        for y in 0..self.map.map_size.1 {
            for x in 0..self.map.map_size.0 {
                self.map.draw_tile(x, y, &mut new_canvas);
            }
        }

        self.canvas = new_canvas;
        let data = self.canvas.access_data();
        data.to_vec()
    }
}

fn main() {
    let mut my_loop = MyGameLoop::default();
    let mut tile = Tile::new(80, 40);
    tile.color = Rgb::BLUE;
    my_loop.map = GameMap {
        tile,
        map_size: (20, 10),
        map_origin: (5, 1),
        shift_x: 0,
        shift_y: 0,
        map_to_screen_transform: [0, 0, 0, 0],
    };
    my_loop.map.calculate_transform();

    let my_conf = BackendConf {
        window_title: "tiled_map".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
