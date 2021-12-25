use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn dist(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        return std::cmp::max(dx, dy);
    }

    fn inline(&self, other: &Point) -> bool {
        return self.x == other.x || self.y == other.y;
    }
}

fn dir(a: i64, b: i64) -> i64 {
    if a > b {
        -1
    } else if a == b {
        0
    } else {
        1
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<(i64, i64), usize>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: HashMap::new(),
        }
    }

    fn plot_pair(&mut self, a: Point, b: Point) {
        let dx = dir(a.x, b.x);
        let dy = dir(a.y, b.y);
        for i in 0..=a.dist(&b) {
            let x = a.x + dx * i;
            let y = a.y + dy * i;
            *self.grid.entry((x, y)).or_insert(0) += 1;
        }
    }

    fn overlap_count(&self) -> usize {
        return self.grid.values().filter(|&v| v >= &2).count();
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let overlap = input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_numeric())
                .map(|c| c.parse::<i64>())
                .filter(|c| c.is_ok())
                .map(|c| c.unwrap())
                .collect::<Vec<_>>()
        })
        .map(|quad| (Point::new(quad[0], quad[1]), Point::new(quad[2], quad[3])))
        // .filter(|(a, b)| a.inline(b))
        .fold(Grid::new(), |mut acc, (a, b)| {
            acc.plot_pair(a, b);
            acc
        });

    println!("{:?}", overlap.overlap_count());
}
