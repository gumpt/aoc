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
    type Item = u32;

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
                return Some(self.grid.points[x as usize][y as usize]);
            }
        }
    }
}

fn main() {
    let input = Grid::new(
        include_str!("../input.txt")
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    );

    // println!("{:?}", input);

    let mut low_levels = 0;
    for i in 0..input.points.len() {
        for j in 0..input.points[i].len() {
            let value = input.points[i][j];
            if input.neighbors(i, j).all(|n| n > value) {
                low_levels += 1 + value;
            }
        }
    }
    println!("PART 1: {}", low_levels);
}
