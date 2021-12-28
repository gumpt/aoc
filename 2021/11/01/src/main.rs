use std::collections::HashSet;

type Coords = (i64, i64);

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<u32>>,
}

impl Grid {
    fn new(input: Vec<Vec<u32>>) -> Grid {
        Grid { points: input }
    }

    fn neighbors(&self, x: usize, y: usize) -> NeighborIter {
        NeighborIter {
            grid_width: self.points.len(),
            grid_height: self.points[0].len(),
            starting_coords: (x as i64, y as i64),
            index: 0,
        }
    }

    fn run_generations(&mut self, n: u32) -> u32 {
        let mut acc = 0;
        for i in 0..n {
            let (gen, all) = self.run_generation();
            println!("GENERATION {}: {}", i + 1, gen);
            println!("{:?}", self.points);
            acc += gen;
            if all {
                break;
            }
        }
        acc
    }

    fn run_generation(&mut self) -> (u32, bool) {
        self.step_one();
        let acc = self.flash_all_and_count();
        let all = self.step_two();
        (acc, all)
    }

    fn step_one(&mut self) {
        for i in 0..self.points.len() {
            for j in 0..self.points[i].len() {
                self.points[i][j] += 1;
            }
        }
    }

    fn flash_all_and_count(&mut self) -> u32 {
        let mut acc = 0;
        let mut stack = Vec::new();
        let mut seen = HashSet::new();

        // Self-flashers
        for i in 0..self.points.len() {
            for j in 0..self.points[i].len() {
                if self.points[i][j] > 9 {
                    acc += 1;
                    seen.insert((i, j));
                    for (x, y) in self.neighbors(i, j) {
                        self.points[x][y] += 1;
                        stack.push((x, y));
                    }
                }
            }
        }

        // Cascade
        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            if seen.contains(&(i, j)) || self.points[i][j] < 10 {
                continue;
            }
            acc += 1;
            self.points[i][j] = 10;
            seen.insert((i, j));

            for (x, y) in self.neighbors(i, j) {
                self.points[x][y] += 1;
                stack.push((x, y));
            }
        }

        acc
    }

    fn step_two(&mut self) -> bool {
        let mut all_flashing = true;
        for i in 0..self.points.len() {
            for j in 0..self.points[i].len() {
                if self.points[i][j] > 9 {
                    self.points[i][j] = 0;
                } else {
                    all_flashing = false;
                }
            }
        }
        all_flashing
    }
}

struct NeighborIter {
    grid_width: usize,
    grid_height: usize,
    starting_coords: Coords,

    index: usize,
}

impl NeighborIter {
    const NEIGHBORS: [Coords; 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let neighbor = NeighborIter::NEIGHBORS.get(self.index)?;
            self.index += 1;

            // Merge
            let (x, y) = (
                self.starting_coords.0 + neighbor.0,
                self.starting_coords.1 + neighbor.1,
            );

            // Bounds check
            if x >= 0 && x < self.grid_width as i64 && y >= 0 && y < self.grid_height as i64 {
                return Some((x as usize, y as usize));
            }
        }
    }
}

fn main() {
    let mut starting_grid = Grid::new(
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect(),
    );

    println!("{:?}", starting_grid);
    println!("PART 1: {}", starting_grid.run_generations(1000));
}
