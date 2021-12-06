use std::error::Error;
use std::str::FromStr;

#[derive(Default)]
pub struct Submarine {
    pub forward: i32,
    pub depth: i32,
    pub aim: i32,
    pub use_aim: bool,
}

impl Submarine {
    pub fn new(use_aim: bool) -> Self {
        Self {
            forward: 0,
            depth: 0,
            aim: 0,
            use_aim,
        }
    }

    fn forward(&mut self, value: i32) {
        match self.use_aim {
            true => {
                self.forward += value;
                self.depth += self.aim * value
            }
            false => self.forward += value,
        }
    }

    fn up(&mut self, value: i32) {
        match self.use_aim {
            true => self.aim -= value,
            false => self.depth -= value,
        }
    }

    fn down(&mut self, value: i32) {
        match self.use_aim {
            true => self.aim += value,
            false => self.depth += value,
        }
    }

    pub fn position(&self) -> i32 {
        self.forward * self.depth
    }

    pub fn move_sub(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(value) => self.forward(value),
            Direction::Up(value) => self.up(value),
            Direction::Down(value) => self.down(value),
        }
    }
}

pub enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = match s.trim().split_once(" ") {
            Some((dir, val)) => (dir, val),
            None => return Err("Cannot split line".into()),
        };

        let value = i32::from_str(value)?;

        let direction = match direction {
            "forward" => Direction::Forward(value),
            "up" => Direction::Up(value),
            "down" => Direction::Down(value),
            _ => return Err("Unknown direction".into()),
        };

        Ok(direction)
    }
}
