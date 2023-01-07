use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day18.txt");
    let volcano = Volcano::new(&input);
    println!("Day 18 part 1: {}", volcano.surface_area());
    println!("Day 18 part 2: {}", volcano.exterior_surface_area());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3 {
    x: u8,
    y: u8,
    z: u8,
}

impl FromStr for Point3 {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .trim()
            .split(',')
            .map(|v| {
                v.parse()
                    .map_err(|err| color_eyre::eyre::eyre!("Cannot parse number: {err}"))
            })
            .collect::<color_eyre::Result<Vec<_>>>()?;
        Ok(Self {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
}
impl Point3 {
    pub const fn manhattan_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) as u32
            + self.y.abs_diff(other.y) as u32
            + self.z.abs_diff(other.z) as u32
    }

    // pub fn neighbours(&self) -> Vec<Self> {
    //     (0..3)
    //         .map(|_| (0..=2))
    //         .multi_cartesian_product()
    //         .map(|offset| Self {
    //             x: self.x + offset[0] - 1,
    //             y: self.y + offset[1] - 1,
    //             z: self.z + offset[2] - 1,
    //         })
    //         .collect()
    // }
}

pub struct Volcano {
    pub droplets: Vec<Point3>,
    x_bounds: (u8, u8),
    y_bounds: (u8, u8),
    z_bounds: (u8, u8),
}

impl Volcano {
    pub fn surface_area(&self) -> usize {
        let mut surface_area = self.droplets.len() * 6;
        for i in 0..self.droplets.len() {
            for j in (i + 1)..self.droplets.len() {
                if self.droplets[i].manhattan_distance(self.droplets[j]) == 1 {
                    surface_area -= 2;
                }
            }
        }
        surface_area
    }

    pub fn exterior_surface_area(&self) -> usize {
        let reachable_by_water = self.can_be_reached_by_water();
        let mut surface_area = 0;
        for droplet in &self.droplets {
            let n = Point3 {
                x: droplet.x - 1,
                y: droplet.y,
                z: droplet.z,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
            let n = Point3 {
                x: droplet.x + 1,
                y: droplet.y,
                z: droplet.z,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
            let n = Point3 {
                x: droplet.x,
                y: droplet.y - 1,
                z: droplet.z,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
            let n = Point3 {
                x: droplet.x,
                y: droplet.y + 1,
                z: droplet.z,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
            let n = Point3 {
                x: droplet.x,
                y: droplet.y,
                z: droplet.z - 1,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
            let n = Point3 {
                x: droplet.x,
                y: droplet.y,
                z: droplet.z + 1,
            };
            if reachable_by_water[self.p2i(n)] {
                surface_area += 1;
            }
        }
        surface_area
    }

    pub fn new(input: &str) -> Self {
        let droplets = input
            .lines()
            .map(Point3::from_str)
            .collect::<color_eyre::Result<Vec<_>>>()
            .unwrap();
        let x_bounds = droplets.iter().map(|p| p.x).minmax().into_option().unwrap();
        let y_bounds = droplets.iter().map(|p| p.y).minmax().into_option().unwrap();
        let z_bounds = droplets.iter().map(|p| p.z).minmax().into_option().unwrap();
        Self {
            droplets,
            x_bounds,
            y_bounds,
            z_bounds,
        }
    }

    const fn p2i(&self, p: Point3) -> usize {
        (p.z + 1 - self.z_bounds.0) as usize
            * ((self.y_bounds.1 + 3 - self.y_bounds.0) as usize)
            * (self.x_bounds.1 + 3 - self.x_bounds.0) as usize
            + (p.y + 1 - self.y_bounds.0) as usize
                * (self.x_bounds.1 + 3 - self.x_bounds.0) as usize
            + (p.x as usize + 1 - self.x_bounds.0 as usize)
    }

    // fn i2p(&self, index: usize) -> Point3 {
    //     let mut index = index;
    //     let divisor = ((self.y_bounds.1 + 3 - self.y_bounds.0) as usize)
    //         * (self.x_bounds.1 + 3 - self.x_bounds.0) as usize;
    //     let z = (index / divisor).try_into().unwrap();
    //     index -= z as usize * divisor;
    //     let divisor = (self.x_bounds.1 + 3 - self.x_bounds.0) as usize;
    //     let y = (index / divisor).try_into().unwrap();
    //     index -= y as usize * divisor;
    //     let x = (index % divisor).try_into().unwrap();
    //     Point3 { x, y, z }
    // }

    // const fn is_ouf_of_bounds(&self, point: &Point3) -> bool {
    //     point.x < self.x_bounds.0
    //         || point.x > self.x_bounds.1
    //         || point.y < self.y_bounds.0
    //         || point.y > self.y_bounds.1
    //         || point.z < self.z_bounds.0
    //         || point.z > self.z_bounds.1
    // }

    fn can_be_reached_by_water(&self) -> Vec<bool> {
        let mut queue = VecDeque::new();
        let mut can_be_reached = vec![
            false;
            (self.x_bounds.1 + 3 - self.x_bounds.0) as usize
                * (self.y_bounds.1 + 3 - self.y_bounds.0) as usize
                * (self.z_bounds.1 + 3 - self.z_bounds.0) as usize
        ];
        let root = Point3 {
            x: self.x_bounds.0 - 1,
            y: self.y_bounds.0 - 1,
            z: self.z_bounds.0 - 1,
        };
        queue.push_back(root);
        let mut explored: HashSet<Point3> = HashSet::new();
        explored.insert(root);
        while let Some(v) = queue.pop_front() {
            can_be_reached[self.p2i(v)] = true;
            if v.x > self.x_bounds.0 - 1 {
                let n = Point3 {
                    x: v.x - 1,
                    y: v.y,
                    z: v.z,
                };
                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
            if v.y > self.y_bounds.0 - 1 {
                let n = Point3 {
                    x: v.x,
                    y: v.y - 1,
                    z: v.z,
                };

                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
            if v.z > self.z_bounds.0 - 1 {
                let n = Point3 {
                    x: v.x,
                    y: v.y,
                    z: v.z - 1,
                };
                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
            if v.x < self.x_bounds.1 + 1 {
                let n = Point3 {
                    x: v.x + 1,
                    y: v.y,
                    z: v.z,
                };
                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
            if v.y < self.y_bounds.1 + 1 {
                let n = Point3 {
                    x: v.x,
                    y: v.y + 1,
                    z: v.z,
                };
                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
            if v.z < self.z_bounds.1 + 1 {
                let n = Point3 {
                    x: v.x,
                    y: v.y,
                    z: v.z + 1,
                };
                if !self.droplets.contains(&n) && explored.insert(n) {
                    // new
                    queue.push_back(n);
                }
            }
        }
        can_be_reached
    }
}

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;

    use crate::util::read_example;

    use super::*;

    #[test]
    fn it_parses_point() {
        let point: Point3 = "1,2,3".parse().unwrap();
        assert_eq!(Point3 { x: 1, y: 2, z: 3 }, point);
    }

    #[test]
    fn it_finds_surface_area() {
        let input = read_example("day18.txt");
        let volcano = Volcano::new(&input);
        assert_eq!(64, volcano.surface_area());
    }

    #[test]
    fn get_bounds() {
        let input = read_input("day18.txt");
        let volcano = Volcano::new(&input);
        dbg!(volcano.droplets.iter().map(|p| p.x).minmax());
        dbg!(volcano.droplets.iter().map(|p| p.y).minmax());
        dbg!(volcano.droplets.iter().map(|p| p.z).minmax());
    }

    #[test]
    fn it_converts_point_to_index() {
        let input = read_example("day18.txt");
        let volcano = Volcano::new(&input);
        let mut i = 0;
        for z in (volcano.z_bounds.0 - 1)..=(volcano.z_bounds.1 + 1) {
            for y in (volcano.y_bounds.0 - 1)..=(volcano.y_bounds.1 + 1) {
                for x in (volcano.x_bounds.0 - 1)..=(volcano.x_bounds.1 + 1) {
                    let point = Point3 { x, y, z };
                    assert_eq!(volcano.p2i(point), i, "Point {point:?} => index {i}");
                    i += 1;
                }
            }
        }
    }

    #[test]
    fn it_finds_if_point_can_be_reached_by_water() {
        let input = read_example("day18.txt");
        let volcano = Volcano::new(&input);
        let can_be_reached = volcano.can_be_reached_by_water();
        assert!(!can_be_reached[volcano.p2i(Point3 { x: 2, y: 2, z: 5 })]);
        assert!(!can_be_reached[volcano.p2i(Point3 { x: 2, y: 2, z: 4 })]);
        assert!(can_be_reached[volcano.p2i(Point3 { x: 2, y: 1, z: 1 })]);
    }

    #[test]
    fn it_finds_exterior_surface_area() {
        let input = read_example("day18.txt");
        let volcano = Volcano::new(&input);
        assert_eq!(58, volcano.exterior_surface_area());
    }
}
