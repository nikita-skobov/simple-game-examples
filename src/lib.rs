pub mod backend;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
        ($i * Canvas::BPP) + ($j * $me.span())
    };
}

impl Canvas {
    // bytes per pixel, always 3. we dont consider alphas here...
    const BPP: usize = 3;

    pub fn new(width: usize, height: usize) -> Canvas {
        let data = vec![0; width * height * Canvas::BPP];
        Canvas {
            data,
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn span(&self) -> usize {
        self.width * Canvas::BPP
    }

    pub fn new_with_color(width: usize, height: usize, color: Rgb) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(color);
        canvas
    }

    pub fn fill(&mut self, color: Rgb) {
        let span = self.span();
        for i in 0..self.width {
            for j in 0..self.height {
                let x_offset = i * Canvas::BPP;
                let y_offset = j * span;
                let red_index = x_offset + y_offset;
                self.set_pixel_from_index(red_index, color);
            }
        }
    }

    /// red_index should be the index of the data vector for
    /// a red value of the pixel we wish to set.
    pub fn set_pixel_from_index(&mut self, red_index: usize, color: Rgb) {
        self.data[red_index] = color.red;
        self.data[red_index + 1] = color.green;
        self.data[red_index + 2] = color.blue;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        let red_index = get_red_index!(self, x, y);
        self.set_pixel_from_index(red_index, color);
    }

    pub fn into_raw(self) -> (usize, usize, Vec<u8>) {
        (self.width, self.height, self.data)
    }
}
