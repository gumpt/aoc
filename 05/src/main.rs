use std::cmp;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Ticket {
    row: usize,
    column: usize,
}

impl Ticket {
    fn new(line: String) -> Ticket {
        let (mut row, mut column) = (0, 0);
        for (i, c) in line.chars().enumerate() {
            match i {
                0..=6 => {
                    row <<= 1;
                    match c {
                        'F' => {}
                        'B' => {
                            row |= 1;
                        }
                        _ => {}
                    }
                }
                _ => {
                    column <<= 1;
                    match c {
                        'R' => {
                            column |= 1;
                        }
                        'L' => {}
                        _ => {}
                    }
                }
            }
        }

        Ticket { row, column }
    }

    fn id(&self) -> usize {
        (self.row * 8) + self.column
    }
}

fn main() {
    let stdin = io::stdin();

    let mut seats = vec![vec![0; 8]; 128];

    let mut max = 0;
    for line in stdin.lock().lines() {
        let ticket = Ticket::new(line.unwrap());
        seats[ticket.row][ticket.column] = 1;
        max = cmp::max(max, ticket.id());
    }

    for (i, row) in seats.iter().enumerate() {
        let mut copy = row.clone();
        copy.retain(|&x| x == 0);
        if copy.len() == 1 {
            for (j, state) in row.iter().enumerate() {
                if state.clone() == 0 {
                    let t = Ticket { row: i, column: j };
                    println!("PART 2: {}", t.id());
                }
            }
        }
    }

    println!("PART 1: {}", max);
}
