use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Relative {
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
enum Instruction {
    Absolute(Cardinal, i64),
    Turn(Relative, i64),
}

use Cardinal::*;
use Instruction::*;
use Relative::*;

// All degrees are multiples of 90, which is wonderful!
impl Cardinal {
    fn turn_left(&self, degrees: i64) -> Cardinal {
        return match degrees {
            90 => match *self {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            180 => match *self {
                North => South,
                West => East,
                South => North,
                East => West,
            },
            270 => match *self {
                North => East,
                West => North,
                South => West,
                East => South,
            },
            _ => *self,
        };
    }

    fn turn_right(&self, degrees: i64) -> Cardinal {
        return match degrees {
            90 => match *self {
                North => East,
                West => North,
                South => West,
                East => South,
            },
            180 => match *self {
                North => South,
                West => East,
                South => North,
                East => West,
            },
            270 => match *self {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            _ => *self,
        };
    }
}

#[derive(Debug)]
struct Ship {
    direction: Cardinal,
    east: i64,
    west: i64,
    north: i64,
    south: i64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: East,
            east: 0,
            west: 0,
            north: 0,
            south: 0,
        }
    }

    fn run(&mut self, action: &Instruction) {
        match action {
            Absolute(cardinal, mag) => match cardinal {
                East => self.east += mag,
                West => self.west += mag,
                North => self.north += mag,
                South => self.south += mag,
            },
            Turn(direction, degrees) => match direction {
                Left => self.direction = self.direction.turn_left(*degrees),
                Right => self.direction = self.direction.turn_right(*degrees),
                Forward => self.run(&Absolute(self.direction, *degrees)),
            },
        }
    }

    fn manhattan(&self) -> i64 {
        // println!("{:?}", self);
        let ew = (self.east - self.west).abs();
        let ns = (self.north - self.south).abs();
        ew + ns
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut ship = Ship::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap().chars().collect::<Vec<_>>();
        let magnitude = l[1..]
            .into_iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let action = match l[0] {
            'N' => Absolute(North, magnitude),
            'S' => Absolute(South, magnitude),
            'W' => Absolute(West, magnitude),
            'E' => Absolute(East, magnitude),
            'L' => Turn(Left, magnitude),
            'R' => Turn(Right, magnitude),
            'F' => Turn(Forward, magnitude),
            _ => continue,
        };

        ship.run(&action);
    }

    println!("PART 1: {:?}", ship.manhattan());
}
