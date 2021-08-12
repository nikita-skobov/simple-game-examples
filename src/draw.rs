use crate::Rgb;

pub trait Draw {
    fn fill(&mut self, color: Rgb);

    /// red_index should be the index of the data vector for
    /// a red value of the pixel we wish to set.
    fn set_pixel_from_index(&mut self, red_index: usize, color: Rgb);

    fn set_pixel(&mut self, x: usize, y: usize, color: Rgb);

    fn access_data(&self) -> &[u8];

    fn draw_horizontal_line(&mut self, y: usize, x1: usize, x2: usize, color: Rgb);

    fn draw_vertical_line(&mut self, x: usize, y1: usize, y2: usize, color: Rgb);

    fn draw_diagonal_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Rgb);
}
