// Inspired by the original "turing drawings":
// https://github.com/maximecb/Turing-Drawings/blob/master/programs.js#L44-L48

use rand::{thread_rng, Rng};

const TILE_SIZE: usize = 64;
const RESTARTS: usize = 20;
const ITERATIONS: usize = 100;

pub const AGENT_STATES: u8 = 8;
pub const LUT_SIZE: usize = AGENT_STATES as usize * 2;

struct Agent {
    x: i32,
    y: i32,
    state: u8,
}

struct Tile([bool; TILE_SIZE * TILE_SIZE]);

impl Tile {
    fn getpx(&self, x: i32, y: i32) -> bool {
        self.0[Self::idx(x, y)]
    }

    fn setpx(&mut self, x: i32, y: i32, value: bool) {
        self.0[Self::idx(x, y)] = value
    }

    fn idx(x: i32, y: i32) -> usize {
        (y.rem_euclid(TILE_SIZE as i32) as usize) * TILE_SIZE
            + x.rem_euclid(TILE_SIZE as i32) as usize
    }

    fn new() -> Self {
        Tile([false; TILE_SIZE * TILE_SIZE])
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut tile = Tile::new();
    let mut agent = Agent {
        x: (TILE_SIZE / 2) as i32,
        y: (TILE_SIZE / 2) as i32,
        state: 0,
    };

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
            tile.setpx(agent.x, agent.y, (command & 0b001) != 0);
            assert!(AGENT_STATES < (1 << (8 - 3)));
            agent.state = (command >> 3).rem_euclid(AGENT_STATES);
        }
    }

    for y in 0..TILE_SIZE as i32 {
        let mut s = String::new();
        for x in 0..TILE_SIZE as i32 {
            s += match tile.getpx(x, y) {
                false => ".",
                true => "#",
            };
        }
        println!("{}", s);
    }
}
