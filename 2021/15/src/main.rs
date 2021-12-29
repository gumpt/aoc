use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Coords = (i64, i64);
type HeapValue = (Reverse<u32>, Coords);

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<u32>>,
}

impl Grid {
    fn new(input: Vec<Vec<u32>>) -> Grid {
        Grid { points: input }
    }

    fn neighbors(&self, (x, y): Coords) -> NeighborIter {
        NeighborIter {
            grid_width: self.points.len(),
            grid_height: self.points[0].len(),
            starting_coords: (x as i64, y as i64),
            index: 0,
        }
    }

    fn manhattan(&self, start: Coords, end: Coords) -> u32 {
        (end.0 + end.1 - start.0 - start.1) as u32
    }

    fn min_risk_to_bottom_right(&self) -> u32 {
        let start = (0, 0);
        let far_coordinate = (
            (self.points.len() - 1) as i64,
            (self.points[0].len() - 1) as i64,
        );
        println!("{:?}", far_coordinate);

        let manhattan = |point| self.manhattan(point, far_coordinate);

        let mut open_set: BinaryHeap<HeapValue> = BinaryHeap::new();
        let mut came_from: HashMap<Coords, Coords> = HashMap::new();
        let mut g_score: HashMap<Coords, u32> = HashMap::new();
        g_score.insert(start, 0);

        open_set.push((Reverse(0), start));

        while !open_set.is_empty() {
            let (path_value, mut current) = open_set.pop().unwrap();
            println!("{:?} {:?}", path_value, current);

            if current == far_coordinate {
                let mut path: Vec<Coords> = vec![current];
                while came_from.contains_key(&current) {
                    current = *came_from.get(&current).unwrap();
                    path.push(current);
                }
                path.reverse();
                let path_value = path
                    .iter()
                    .fold(0, |acc, &(i, j)| acc + self.points[i as usize][j as usize])
                    - self.points[0][0];
                println!("PATH: {:?}", path);
                return path_value;
            }

            for neighbor in self.neighbors(current) {
                let (x, y) = current;
                let current_risk = self.points[x as usize][y as usize];
                let tentative_g_score = g_score.get(&current).unwrap() + current_risk;

                if tentative_g_score < *g_score.entry(neighbor).or_insert(u32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push((Reverse(tentative_g_score + manhattan(neighbor)), neighbor));
                }
            }
        }

        0
    }
}

struct NeighborIter {
    grid_width: usize,
    grid_height: usize,
    starting_coords: Coords,

    index: usize,
}

impl NeighborIter {
    const NEIGHBORS: [Coords; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
}

impl Iterator for NeighborIter {
    type Item = (i64, i64);

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
                return Some((x, y));
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<_>>();

    let part_one_grid = Grid::new(input.clone());
    println!("PART ONE: {}", part_one_grid.min_risk_to_bottom_right());
    let height = input.len();
    let width = input[0].len();

    let mut big_points: Vec<Vec<u32>> = vec![vec![0; width * 5]; height * 5];

    for i in 0..5 {
        let mut adjusted_i = i * height;
        for x in 0..height {
            for j in 0..5 {
                let mut adjusted_j = j * width;
                for y in 0..width {
                    let current_i = adjusted_i + x;
                    let current_j = adjusted_j + y;
                    let mut risk = (input[x as usize][y as usize] + j as u32 + i as u32) % 9;
                    if risk == 0 {
                        risk = 9;
                    }
                    // println!("{} => {} ({},{})", input[x][y], risk, current_i, current_j);

                    big_points[current_i][current_j] = risk;
                }
            }
        }
    }

    let big_grid = Grid::new(big_points);
    println!("PART TWO: {}", big_grid.min_risk_to_bottom_right());
}
