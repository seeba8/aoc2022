use std::str::FromStr;

/// # Panics
/// This may panic if there is a problem with the file
#[must_use] pub fn read_input(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("resources/").join(filename)).unwrap()
}

/// # Panics
/// This may panic if there is a problem with the file
#[must_use] pub fn read_example(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("examples/").join(filename)).unwrap()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(color_eyre::eyre::eyre!("Cannot parse direction '{s}'"))
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Default)]
struct Spiral {
    width: isize,
    height: isize,
    current_circle: isize,
    current_index: isize,
    current_direction: Direction,
    visited: isize,
}

impl Spiral {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        Self {
            width: width as isize,
            height: height as isize,
            ..Default::default()
        }
    }
}

impl Iterator for Spiral {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visited >= self.width * self.height {
            return None;
        }
        self.visited += 1;
        if self.visited == 1 {
            return Some(0);
        }
        match self.current_direction {
            Direction::Up => {
                if self.current_index - self.current_circle - (1 + self.current_circle) * self.width
                    <= 0
                {
                    self.current_direction = Direction::Right;
                    self.current_circle += 1;
                    self.current_index += 1;
                } else {
                    self.current_index -= self.width;
                }
                Some(self.current_index.unsigned_abs())
            }
            Direction::Right => {
                if (self.current_index + 1 + self.current_circle) % self.width == 0 {
                    self.current_direction = Direction::Down;
                    self.current_index += self.width;
                } else {
                    self.current_index += 1;
                }
                Some(self.current_index.unsigned_abs())
            }
            Direction::Down => {
                if self.current_index + (1 + self.current_circle) * self.width
                    >= self.width * self.height
                {
                    self.current_direction = Direction::Left;
                    self.current_index -= 1;
                } else {
                    self.current_index += self.width;
                }
                Some(self.current_index.unsigned_abs())
            }
            Direction::Left => {
                if (self.current_index - self.current_circle) % self.width == 0 {
                    self.current_direction = Direction::Up;
                    self.current_index -= self.width;
                } else {
                    self.current_index -= 1;
                }
                Some(self.current_index.unsigned_abs())
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn it_spirals() {
        let spiral = Spiral::new(5, 5);
        let indices: Vec<usize> = spiral.collect();
        /*
         *  0--1--2--3--4
         *              |
         *  5--6--7--8  9
         *  |        |  |
         * 10 11-12 13 14
         *  |  |     |  |
         * 15 16-17-18 19
         *  |           |
         * 20-21-22-23-24
         */
        assert_eq!(
            vec![
                0, 1, 2, 3, 4, 9, 14, 19, 24, 23, 22, 21, 20, 15, 10, 5, 6, 7, 8, 13, 18, 17, 16,
                11, 12
            ],
            indices
        );
    }
}