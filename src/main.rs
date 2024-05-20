// Inspired by the original "turing drawings":
// https://github.com/maximecb/Turing-Drawings/blob/master/programs.js#L44-L48

use image::Luma;
use rand::{thread_rng, Rng};

mod tile;
use tile::TILE_SIZE;

const RESTARTS: usize = 500000;
const ITERATIONS: usize = 200;

pub const AGENT_STATES: u8 = 31;
pub const LUT_SIZE: usize = AGENT_STATES as usize * 2;

struct Agent {
    x: i32,
    y: i32,
    state: u8,
}

fn main() {
    let mut rng = thread_rng();
    let mut tile = tile::Tile::new();
    let mut agent = Agent {
        x: (TILE_SIZE / 2) as i32,
        y: (TILE_SIZE / 2) as i32,
        state: 0,
    };

    let mut canvas =
        image::ImageBuffer::<Luma<f32>, Vec<f32>>::new(TILE_SIZE as u32, TILE_SIZE as u32);

    println!("Generating...");
    for _ in 0..RESTARTS {
        let lut: [_; LUT_SIZE] = std::array::from_fn(|_| rng.gen::<u8>());
        for _ in 0..ITERATIONS {
            assert!(LUT_SIZE < 256);
            let key: u8 = (tile.getpx(agent.x, agent.y) as u8) + (agent.state << 1);
            let command = lut[key as usize];
            let (dx, dy) = match command & 0b11 {
                0 => (1, 0),
                1 => (-1, 0),
                2 => (0, 1),
                3 => (0, -1),
                _ => unreachable!(),
            };
            agent.x += dx;
            agent.y += dy;
            agent.x = agent.x.rem_euclid(TILE_SIZE as i32);
            agent.y = agent.y.rem_euclid(TILE_SIZE as i32);

            let white = (command & 0b001) != 0;
            tile.setpx(agent.x, agent.y, white);
            if white {
                let px = canvas.get_pixel_mut(agent.x as u32, agent.y as u32);
                *px = Luma([px[0] + 0.1]);
            }

            assert!(AGENT_STATES < (1 << (8 - 3)));
            agent.state = (command >> 3).rem_euclid(AGENT_STATES);
        }
    }

    println!("Rendering raw output...");
    tile.render_image().save("output_raw.png").unwrap();

    println!("Rendering fancy output...");
    fn linear_to_srgb_gamma(l: f32) -> f32 {
        if l <= 0.0031308 {
            l * 12.92
        } else {
            1.055 * l.powf(1.0 / 2.4) - 0.055
        }
    }
    let canvas_gray = image::GrayImage::from_fn(TILE_SIZE as u32, TILE_SIZE as u32, |x, y| {
        let l = canvas.get_pixel(x, y)[0];
        let l = linear_to_srgb_gamma(l);
        Luma([(l.clamp(0.0, 1.0) * 255.0).round() as u8])
    });
    canvas_gray.save("output.png").unwrap();

    println!("Done!");
}
