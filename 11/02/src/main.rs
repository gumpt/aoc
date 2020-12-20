#[macro_use]
extern crate lazy_static;
use std::io::BufRead;

#[derive(PartialEq, Clone, Debug)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl SeatState {
    fn from_char(c: char) -> SeatState {
        return match c {
            'L' => SeatState::Empty,
            '#' => SeatState::Occupied,
            _ => SeatState::Floor,
        };
    }

    fn to_char(&self) -> char {
        return match self {
            SeatState::Empty => 'L',
            SeatState::Occupied => '#',
            _ => '.',
        };
    }

    fn next(&self, neighbors: usize) -> SeatState {
        return match self {
            SeatState::Floor => SeatState::Floor,
            SeatState::Occupied => {
                if neighbors >= 5 {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
            SeatState::Empty => {
                if neighbors == 0 {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }
        };
    }
}

struct Grid {
    occupied_seats: usize,
    row_length: usize,
    current: Vec<Vec<SeatState>>,
    next: Vec<Vec<SeatState>>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            occupied_seats: 0,
            row_length: 0,
            current: vec![],
            next: vec![],
        }
    }

    fn print(&self) {
        println!("GRID!!!!!");
        for i in 0..self.current.len() {
            let line = self.current[i]
                .iter()
                .map(|x| x.to_char())
                .collect::<String>();
            println!("{:?}", line);
        }
        println!("GRID!!!!!");
        println!("");
    }

    fn add_line(&mut self, line: &str) {
        let chars: Vec<SeatState> = line.chars().map(|c| SeatState::from_char(c)).collect();

        if self.row_length == 0 {
            self.row_length = chars.len();
        }

        if chars.len() != self.row_length {
            panic!("ack!");
        }

        let occupied = chars
            .iter()
            .filter(|&c| *c == SeatState::Occupied)
            .collect::<Vec<&SeatState>>()
            .len();

        self.occupied_seats += occupied;
        self.current.push(chars);
    }

    fn advance_round(&mut self) {
        let mut next = vec![];
        for line in self.current.iter() {
            next.push(line.clone());
        }

        let mut occupied = 0;
        for i in 0..self.current.len() {
            for j in 0..self.current[i].len() {
                next[i][j] = self.current[i][j].next(self.occupied_neighbors(i, j));
                if next[i][j] == SeatState::Occupied {
                    occupied += 1;
                }
            }
        }

        self.occupied_seats = occupied;
        self.current = next;
    }

    fn occupied_neighbors(&self, i: usize, j: usize) -> usize {
        lazy_static! {
            static ref TRANSFORMS: Vec<(i64, i64)> = vec![
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

        let mut occupied_neighbors = 0;
        for (delta_x, delta_y) in TRANSFORMS.iter() {
            let (mut old_x, mut old_y) = (i as i64, j as i64);

            loop {
                let (new_x, new_y) = (old_x + delta_x, old_y + delta_y);
                if new_x < 0 || new_y < 0 {
                    break;
                }

                let (index_x, index_y) = (new_x as usize, new_y as usize);
                if index_x >= self.current.len() || index_y >= self.row_length {
                    break;
                }

                match self.current[index_x][index_y] {
                    SeatState::Occupied => {
                        occupied_neighbors += 1;
                        break;
                    }
                    SeatState::Empty => break,
                    SeatState::Floor => {
                        old_x = new_x;
                        old_y = new_y;
                        continue;
                    }
                }
            }
        }

        occupied_neighbors
    }
}

fn main() {
    let mut grid = Grid::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let parsed = line.unwrap();
        grid.add_line(&parsed);
    }

    grid.print();
    println!("{}", grid.occupied_neighbors(4, 3));

    loop {
        let occupied = grid.occupied_seats;
        grid.advance_round();
        // Because it's fun!!
        grid.print();
        if occupied == grid.occupied_seats {
            break;
        }
    }

    println!("PART 2: {}", grid.occupied_seats);
}
