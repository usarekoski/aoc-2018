use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    x: u64,
    y: u64,
}

impl FromStr for Coordinate {
    type Err = Box<::std::error::Error>;

    // input: "1, 2"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+), (\d+)$").unwrap();
        }
        let cap = RE.captures(s).unwrap();

        Ok(Coordinate {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
        })
    }
}

impl Coordinate {
    fn manhattan_distance(self, other: Self) -> u64 {
        let x1 = self.x as i64;
        let x2 = other.x as i64;
        let y1 = self.y as i64;
        let y2 = other.y as i64;

        (i64::abs(x1 - x2) + i64::abs(y1 - y2)) as u64
    }
}

fn find_closest_point(coordinates: &Vec<Coordinate>, c: Coordinate) -> Option<usize> {
    let distances = coordinates
        .iter()
        .map(|&c1| c.manhattan_distance(c1))
        .enumerate();
    let mut min = None;
    let mut min_idx = None;
    let mut is_tied = false;

    for (idx, d) in distances {
        if let Some(m) = min {
            if d < m {
                min = Some(d);
                min_idx = Some(idx);
                is_tied = false;
            } else if d == m {
                is_tied = true;
            }
        } else {
            min = Some(d);
            min_idx = Some(idx);
        }
    }

    if !is_tied {
        min_idx
    } else {
        None
    }
}

// Solution: 4342
pub fn solve1(coordinates: Vec<Coordinate>) -> u64 {
    const GRID_SIZE: u64 = 400;
    let mut areas: Vec<u64> = Vec::new();
    areas.resize(coordinates.len(), 0);
    // areas that are infinite
    let mut unqualified: HashSet<usize> = HashSet::new();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let c = Coordinate { x, y };
            if let Some(idx) = find_closest_point(&coordinates, c) {
                areas[idx] += 1;
                // Area is infinite if it touches the border of grid.
                if x == 0 || x == GRID_SIZE - 1 || y == 0 || y == GRID_SIZE - 1 {
                    unqualified.insert(idx);
                }
            }
        }
    }

    let (_, area) = areas
        .iter()
        .enumerate()
        .filter(|(idx, _area)| !unqualified.contains(idx))
        .max_by_key(|(_, &area)| area)
        .unwrap();

    *area
}

fn distance_qualifies(coordinates: &Vec<Coordinate>, c: Coordinate, distance_limit: u64) -> bool {
    let sum: u64 = coordinates.iter().map(|&c1| c.manhattan_distance(c1)).sum();
    sum < distance_limit
}

fn region_size(coordinates: Vec<Coordinate>, distance_limit: u64) -> u64 {
    const GRID_SIZE: u64 = 400;
    let mut region_size: u64 = 0;

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let c = Coordinate { x, y };
            if distance_qualifies(&coordinates, c, distance_limit) {
                region_size += 1;
            }
        }
    }

    region_size
}

// Solution: 42966
pub fn solve2(coordinates: Vec<Coordinate>) -> u64 {
    region_size(coordinates, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]
            .iter()
            .map(|&s| String::from(s).parse::<Coordinate>().unwrap())
            .collect();
        assert_eq!(solve1(input), 17);
    }

    #[test]
    fn test2() {
        let input = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]
            .iter()
            .map(|&s| String::from(s).parse::<Coordinate>().unwrap())
            .collect();
        assert_eq!(region_size(input, 32), 16);
    }
}
