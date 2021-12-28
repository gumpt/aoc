use std::collections::HashMap;

type Coords = (usize, usize);

struct Paper {
    dots: HashMap<Coords, bool>,
}

impl Paper {
    fn new() -> Paper {
        Paper {
            dots: HashMap::new(),
        }
    }

    fn add_coord(&mut self, xy: Coords) {
        self.dots.insert(xy, true);
    }

    fn fold_x(&self, axis: i64) -> Paper {
        let mut dots = HashMap::new();
        // Each coordinate is moved to an offset of the axis where
        // the offset is its original distance from it.
        // Only positive values live here!
        for (x, y) in self.dots.keys() {
            let mut x_prime = *x;
            if *x > axis as usize {
                x_prime = (axis - (*x as i64 - axis)).abs() as usize;
            }
            dots.insert((x_prime, *y), true);
        }

        Paper { dots }
    }

    fn fold_y(&self, axis: i64) -> Paper {
        let mut dots = HashMap::new();
        for (x, y) in self.dots.keys() {
            let mut y_prime = *y;
            if *y > axis as usize {
                y_prime = (axis - (*y as i64 - axis)).abs() as usize;
            }
            dots.insert((*x, y_prime), true);
        }

        Paper { dots }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut parts = input.split("\n\n");

    let mut paper = Paper::new();
    let coords = parts.next().unwrap();
    for line in coords.lines() {
        // println!("{}", line);
        let split: Vec<usize> = line.split(",").map(|c| c.parse().unwrap()).collect();
        let (x, y) = (split[0], split[1]);
        paper.add_coord((x, y));
    }

    for fold in parts.next().unwrap().lines() {
        let split = fold.split(" ").last().unwrap();

        let mut instruction = split.split("=");
        let (axis, value) = (
            instruction.next().unwrap(),
            instruction.next().unwrap().parse::<i64>().unwrap(),
        );
        paper = match axis {
            "x" => paper.fold_x(value),
            "y" => paper.fold_y(value),
            _ => unreachable!(),
        };
    }

    let max_x = paper.dots.keys().map(|(x, _)| x).max().unwrap();
    let max_y = paper.dots.keys().map(|(_, y)| y).max().unwrap();
    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];
    for line in &mut grid {
        line.push('\n');
    }
    for (x, y) in paper.dots.keys().into_iter() {
        grid[*y][*x] = '#';
    }

    println!("{}", grid.into_iter().flatten().collect::<String>());
}
