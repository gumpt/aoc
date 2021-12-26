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
            grid: self,
            starting_coords: (x as i64, y as i64),
            index: 0,
        }
    }

    fn get_basin_size(&self, x: usize, y: usize) -> usize {
        let mut border = vec![(x, y)];
        let mut surface = HashSet::new();

        while !border.is_empty() {
            let (i, j) = border.pop().unwrap();
            if surface.contains(&(i, j)) || self.points[i][j] == 9 {
                continue;
            }
            surface.insert((i, j));
            border.extend(self.neighbors(i, j));
        }

        surface.len()
    }
}

struct NeighborIter<'a> {
    grid: &'a Grid,
    starting_coords: Coords,

    index: usize,
}

impl NeighborIter<'_> {
    const NEIGHBORS: [Coords; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
}

impl<'a> Iterator for NeighborIter<'a> {
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
            if x >= 0
                && x < self.grid.points.len() as i64
                && y >= 0
                && y < self.grid.points[x as usize].len() as i64
            {
                return Some((x as usize, y as usize));
            }
        }
    }
}

fn main() {
    let grid = Grid::new(
        include_str!("../input.txt")
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    );

    let mut basins = Vec::new();
    let mut risk = 0;
    for i in 0..grid.points.len() {
        for j in 0..grid.points[i].len() {
            let value = grid.points[i][j];
            if grid.neighbors(i, j).all(|(x, y)| grid.points[x][y] > value) {
                risk += 1 + value;
                basins.push(grid.get_basin_size(i, j));
            }
        }
    }
    println!("PART 1: {}", risk);
    basins.sort();
    println!("PART 2: {}", basins.iter().rev().take(3).product::<usize>());
}
