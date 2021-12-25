use std::io::{self, BufRead};

#[derive(Debug)]
struct BitCount {
    zeroes: u64,
    ones: u64,
}

impl BitCount {
    fn new() -> BitCount {
        BitCount { zeroes: 0, ones: 0 }
    }

    fn add_char(&mut self, c: &char) {
        match c {
            '0' => self.zeroes += 1,
            '1' => self.ones += 1,
            _ => println!("NUPE"),
        }
    }

    fn most_used(&self) -> u8 {
        if self.zeroes > self.ones {
            return '0' as u8;
        }
        return '1' as u8;
    }

    fn least_used(&self) -> u8 {
        if self.zeroes > self.ones {
            return '1' as u8;
        }
        return '0' as u8;
    }
}

#[derive(Debug)]
struct Grid {
    initialized: bool,
    columns: Vec<BitCount>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            initialized: false,
            columns: Vec::new(),
        }
    }

    fn init_columns(&mut self, row_length: usize) {
        self.columns = Vec::new();
        for _ in 0..row_length {
            self.columns.push(BitCount::new());
        }
        self.initialized = true;
    }

    fn add_row(&mut self, row: &str) {
        for (i, c) in row.chars().enumerate() {
            self.columns[i].add_char(&c);
        }
    }

    fn gamma(&self) -> i64 {
        let gamma_vec = self
            .columns
            .iter()
            .map(|c| c.most_used())
            .collect::<Vec<u8>>();
        let gamma_str = std::str::from_utf8(&gamma_vec).unwrap();
        let gamma = i64::from_str_radix(gamma_str, 2);
        return gamma.unwrap();
    }

    fn epsilon(&self) -> i64 {
        let ep_vec = self
            .columns
            .iter()
            .map(|c| c.least_used())
            .collect::<Vec<u8>>();
        let ep_str = std::str::from_utf8(&ep_vec).unwrap();
        let ep = i64::from_str_radix(ep_str, 2);
        return ep.unwrap();
    }
}

fn main() {
    let mut grid = Grid::new();
    let stdin = io::stdin();
    for unwrapped in stdin.lock().lines() {
        let line = unwrapped.unwrap();

        if !grid.initialized {
            grid.init_columns(line.len());
        }
        grid.add_row(&line);
    }
    println!(
        "{} {} {}",
        grid.gamma(),
        grid.epsilon(),
        grid.gamma() * grid.epsilon()
    );
}
