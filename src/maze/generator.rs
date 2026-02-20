//! Maze generation using recursive backtracker algorithm.
//! Produces a 20x20 grid where each cell tracks which walls are open.

pub const MAZE_WIDTH: usize = 20;
pub const MAZE_HEIGHT: usize = 20;

pub const NORTH: u8 = 0b0001;
pub const SOUTH: u8 = 0b0010;
pub const EAST:  u8 = 0b0100;
pub const WEST:  u8 = 0b1000;

/// A single maze cell storing which walls are open as a bitmask.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub passages: u8,
}

impl Cell {
    /// Returns true if the given direction is open.
    pub fn is_open(&self, direction: u8) -> bool {
        self.passages & direction != 0
    }
}

/// The full maze grid stored as a Bevy resource.
#[derive(bevy::prelude::Resource, Debug)]
pub struct Maze {
    pub cells: [[Cell; MAZE_WIDTH]; MAZE_HEIGHT],
}

/// Simple LCG pseudo-random number generator seeded by system time.
struct Rng(u64);

impl Rng {
    /// Creates a new RNG with a fixed seed for reproducibility.
    fn new(seed: u64) -> Self { Rng(seed) }

    /// Returns the next pseudo-random usize in range [0, max).
    fn next_usize(&mut self, max: usize) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % max
    }
}

impl Maze {
    /// Generates a new maze using recursive backtracker (DFS).
    pub fn generate(seed: u64) -> Self {
        let mut cells = [[Cell::default(); MAZE_WIDTH]; MAZE_HEIGHT];
        let mut visited = [[false; MAZE_WIDTH]; MAZE_HEIGHT];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut rng = Rng::new(seed);

        visited[0][0] = true;
        stack.push((0, 0));

        while let Some(&(x, y)) = stack.last() {
            let neighbors = unvisited_neighbors(x, y, &visited);
            if neighbors.is_empty() {
                stack.pop();
            } else {
                let idx = rng.next_usize(neighbors.len());
                let (nx, ny, dir, opposite) = neighbors[idx];
                cells[y][x].passages |= dir;
                cells[ny][nx].passages |= opposite;
                visited[ny][nx] = true;
                stack.push((nx, ny));
            }
        }

        Maze { cells }
    }
}

/// Returns all unvisited neighbors of (x, y) with direction flags.
fn unvisited_neighbors(
    x: usize,
    y: usize,
    visited: &[[bool; MAZE_WIDTH]; MAZE_HEIGHT],
) -> Vec<(usize, usize, u8, u8)> {
    let mut neighbors = Vec::new();
    if y > 0 && !visited[y - 1][x] {
        neighbors.push((x, y - 1, NORTH, SOUTH));
    }
    if y + 1 < MAZE_HEIGHT && !visited[y + 1][x] {
        neighbors.push((x, y + 1, SOUTH, NORTH));
    }
    if x + 1 < MAZE_WIDTH && !visited[y][x + 1] {
        neighbors.push((x + 1, y, EAST, WEST));
    }
    if x > 0 && !visited[y][x - 1] {
        neighbors.push((x - 1, y, WEST, EAST));
    }
    neighbors
}