pub mod backend;

#[derive(Default)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
    pub bpp: usize,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(orig: (usize, usize)) -> Self {
        Point {
            x: orig.0,
            y: orig.1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LineSegment {
    pub p1: Point,
    pub p2: Point,
}

impl LineSegment {
    pub fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
        (c.y as isize - a.y as isize) * (b.x as isize - a.x as isize) > (b.y as isize - a.y as isize) * (c.x as isize - a.x as isize)
    }

    pub fn intersects(&self, ls2: LineSegment) -> bool {
        let a = &self.p1;
        let b = &self.p2;
        let c = &ls2.p1;
        let d = &ls2.p2;
        LineSegment::ccw(a, c, d) != LineSegment::ccw(b, c, d)
            && LineSegment::ccw(a, b, c) != LineSegment::ccw(a, b, d)
    }
}

impl From<(Point, Point)> for LineSegment {
    fn from(orig: (Point, Point)) -> Self {
        LineSegment {
            p1: orig.0,
            p2: orig.1,
        }
    }
}

impl Rgb {
    pub const RED: Rgb = Rgb { red: 255, green: 0, blue: 0 };
    pub const BLUE: Rgb = Rgb { red: 0, green: 0, blue: 255 };
    pub const GREEN: Rgb = Rgb { red: 0, green: 255, blue: 0 };
    pub const BLACK: Rgb = Rgb { red: 0, green: 0, blue: 0 };
    pub const WHITE: Rgb = Rgb { red: 255, green: 255, blue: 255 };
}


macro_rules! get_red_index {
    ($me:tt, $i:tt, $j:tt) => {
        ($i * $me.bpp) + ($j * $me.span())
    };
}

impl Canvas {
    pub fn new(width: usize, height: usize, bpp: usize,) -> Canvas {
        let data = vec![0; width * height * bpp];
        Canvas {
            data,
            width,
            height,
            bpp,
        }
    }

    #[inline(always)]
    pub fn span(&self) -> usize {
        self.width * self.bpp
    }

    pub fn new_with_color(width: usize, height: usize, color: Rgb, bpp: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height, bpp);
        canvas.fill(color);
        canvas
    }

    pub fn fill(&mut self, color: Rgb) {
        let span = self.span();
        for i in 0..self.width {
            for j in 0..self.height {
                let x_offset = i * self.bpp;
                let y_offset = j * span;
                let red_index = x_offset + y_offset;
                self.set_pixel_from_index(red_index, color);
            }
        }
    }

    /// red_index should be the index of the data vector for
    /// a red value of the pixel we wish to set.
    #[inline(always)]
    pub fn set_pixel_from_index(&mut self, red_index: usize, color: Rgb) {
        self.data[red_index] = color.red;
        self.data[red_index + 1] = color.green;
        self.data[red_index + 2] = color.blue;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        if x >= self.width || y >= self.height { return; }
        let red_index = get_red_index!(self, x, y);
        self.set_pixel_from_index(red_index, color);
    }

    pub fn into_raw(self) -> (usize, usize, Vec<u8>) {
        (self.width, self.height, self.data)
    }

    pub fn access_data(&self) -> &[u8] {
        &self.data
    }

    pub fn draw_horizontal_line(&mut self, y: usize, x1: usize, x2: usize, color: Rgb) {
        let span = self.span();
        let y_offset = y * span;
        for i in x1..x2 {
            let x_offset = i * self.bpp;
            let red_index = x_offset + y_offset;
            self.set_pixel_from_index(red_index, color);
        }
    }

    pub fn draw_vertical_line(&mut self, x: usize, y1: usize, y2: usize, color: Rgb) {
        let span = self.span();
        let x_offset = x * self.bpp;
        for j in y1..y2 {
            let y_offset = j * span;
            let red_index = x_offset + y_offset;
            self.set_pixel_from_index(red_index, color);
        }
    }

    pub fn draw_diagonal_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Rgb) {
        let (distance_x, negative_x) = if x2 > x1 {
            (x2 - x1, false)
        } else { (x1 - x2, true) };
        let (distance_y, negative_y) = if y2 > y1 {
            (y2 - y1, false)
        } else { (y1 - y2, true) };
        let num_steps = if distance_x > distance_y {
            distance_x
        } else {
            distance_y
        };
        let mut step_x = distance_x as f32 / num_steps as f32;
        let mut step_y = distance_y as f32 / num_steps as f32;
        if negative_x { step_x *= -1.0 }
        if negative_y { step_y *= -1.0 }
        let mut next_x = x1 as f32;
        let mut next_y = y1 as f32;
        for _ in 0..num_steps {
            self.set_pixel(next_x as usize, next_y as usize, color);
            next_x += step_x;
            next_y += step_y;
        }
    }
}
