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

#[derive(Debug)]
struct Position {
    direction: Cardinal,
    east: i64,
    west: i64,
    north: i64,
    south: i64,
}

struct ShipAndWaypoint {
    ship: Position,
    waypoint: Position,
}

impl ShipAndWaypoint {
    fn new() -> ShipAndWaypoint {
        ShipAndWaypoint {
            waypoint: Position {
                direction: East,
                east: 10,
                north: 1,
                south: 0,
                west: 0,
            },
            ship: Position {
                direction: East,
                east: 0,
                north: 0,
                west: 0,
                south: 0,
            },
        }
    }

    fn run(&mut self, action: &Instruction) {
        match action {
            Absolute(cardinal, mag) => match cardinal {
                East => self.waypoint.east += mag,
                West => self.waypoint.west += mag,
                North => self.waypoint.north += mag,
                South => self.waypoint.south += mag,
            },
            Turn(direction, degrees) => match direction {
                Left => {
                    for _ in 0..(degrees / 90) {
                        let new_position = Position {
                            direction: self.waypoint.direction,
                            east: self.waypoint.south,
                            north: self.waypoint.east,
                            west: self.waypoint.north,
                            south: self.waypoint.west,
                        };
                        self.waypoint = new_position;
                    }
                }
                Right => {
                    for _ in 0..(degrees / 90) {
                        let new_position = Position {
                            direction: self.waypoint.direction,
                            east: self.waypoint.north,
                            north: self.waypoint.west,
                            west: self.waypoint.south,
                            south: self.waypoint.east,
                        };
                        self.waypoint = new_position;
                    }
                }
                Forward => {
                    for _ in 0..*degrees {
                        self.ship.north += self.waypoint.north;
                        self.ship.south += self.waypoint.south;
                        self.ship.east += self.waypoint.east;
                        self.ship.west += self.waypoint.west;
                    }
                }
            },
        }
    }

    fn manhattan(&self) -> i64 {
        // println!("{:?}", self);
        let ew = (self.ship.east - self.ship.west).abs();
        let ns = (self.ship.north - self.ship.south).abs();
        ew + ns
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut ship_and_waypoint = ShipAndWaypoint::new();
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

        ship_and_waypoint.run(&action);
    }

    println!("PART 2: {:?}", ship_and_waypoint.manhattan());
}
