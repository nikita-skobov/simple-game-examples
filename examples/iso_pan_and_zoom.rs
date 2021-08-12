use simple_game_examples::backend;
use simple_game_examples::{Rgb, Canvas};
use backend::{GameLoop, Backend, BackendConf, Event};
use simple_game_examples::{world_screen::WorldScreen, draw::Draw, Point, LineSegment};

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

    pub fn draw_tile<D: Draw>(&self, x: usize, y: usize, canvas: &mut D) {
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
    pub is_scrolling_up: bool,
    pub is_scrolling_down: bool,
    pub mouse_is_dragging: bool,
    pub mouse_was_clicked: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,

    canvas: WorldScreen,

    map: GameMap,
}

impl GameLoop for MyGameLoop {
    fn update(&mut self, events: Vec<Event>) {
        self.mouse_was_clicked = false;
        let mut last_pos = None;
        let mut is_scrolling_up = false;
        let mut is_scrolling_down = false;
        for ev in events {
            let (mx, my) = match ev {
                Event::MouseMove { x, y } => (x, y),
                Event::MouseDown { button, x, y } => {
                    if self.mouse_is_dragging { continue; }
                    if let backend::MouseButton::Left = button {
                        self.mouse_is_dragging = true;
                        self.canvas.reset_pan(x, y);
                    }

                    (x, y)
                }
                Event::MouseScroll { up } => {
                    if up {
                        is_scrolling_up = true;
                    } else {
                        is_scrolling_down = true;
                    }
                    continue;
                }
                Event::MouseUp { button, x, y } => {
                    if let backend::MouseButton::Left = button {
                        self.mouse_is_dragging = false;
                    } else {
                        self.mouse_was_clicked = true;
                    }
                    (x, y)
                }
                _ => { continue; }
            };
            last_pos = Some((mx, my));
        }
        self.is_scrolling_down = is_scrolling_down;
        self.is_scrolling_up = is_scrolling_up;
        if let Some(new_pos) = last_pos {
            self.mouse_x = new_pos.0;
            self.mouse_y = new_pos.1;
        }
    }

    fn draw(&mut self) -> backend::TextureUpdate {
        if self.mouse_is_dragging {
            self.canvas.pan_to(self.mouse_x, self.mouse_y);
        }

        let (_mouse_after_x, _mouse_after_y) = self.canvas.handle_scroll(
            (self.mouse_x, self.mouse_y),
            self.is_scrolling_up, self.is_scrolling_down
        );

        if self.mouse_was_clicked {
            // self.selected_x = mouse_after_x;
            // self.selected_y = mouse_after_y;
        }

        // always re draw the tiles:
        self.canvas.fill(Rgb::WHITE);
        for y in 0..self.map.map_size.1 {
            for x in 0..self.map.map_size.0 {
                self.map.draw_tile(x, y, &mut self.canvas);
            }
        }



        backend::TextureUpdate::UpdateWhole(self.canvas.access_data())
    }

    fn init_canvas(&mut self, width: usize, height: usize, bpp: usize) -> Vec<u8> {
        let color = Rgb::WHITE;
        let new_canvas = Canvas::new_with_color(width, height, color, bpp);
        self.canvas.canvas = new_canvas;
        self.canvas.screen_width = width;
        self.canvas.screen_height = height;
        self.canvas.scale_x = 1.0;
        self.canvas.scale_y = 1.0;
        self.canvas.scale_factor_up = 1.040;
        self.canvas.scale_factor_down = 0.960;
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
        window_title: "iso_pan_and_zoom".into(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
    };
    backend::MQBackend::start(my_conf, my_loop);
}
