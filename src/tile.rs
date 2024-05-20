use image::Luma;

pub const TILE_SIZE: usize = 8192;

pub struct Tile(Vec<bool>);

impl Tile {
    pub fn new() -> Self {
        Tile(vec![false; TILE_SIZE * TILE_SIZE])
    }

    pub fn getpx(&self, x: i32, y: i32) -> bool {
        self.0[Self::idx(x, y)]
    }

    pub fn setpx(&mut self, x: i32, y: i32, value: bool) {
        self.0[Self::idx(x, y)] = value
    }

    fn idx(x: i32, y: i32) -> usize {
        debug_assert!(x >= 0 && x < TILE_SIZE as i32);
        debug_assert!(y >= 0 && y < TILE_SIZE as i32);
        (y as usize) * TILE_SIZE + x as usize
    }

    pub fn render_image(&self) -> image::GrayImage {
        image::GrayImage::from_fn(TILE_SIZE as u32, TILE_SIZE as u32, |x, y| {
            match self.getpx(x as i32, y as i32) {
                false => Luma([0]),
                true => Luma([255]),
            }
        })
    }
}
