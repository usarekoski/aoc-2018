use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    z: i64,
    y: i64,
    x: i64,
}

impl Pos {
    fn new(x: i64, y: i64, z: i64) -> Pos {
        Pos { x, y, z }
    }

    fn manhattan_distance(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn scaled(&self, scale: i64) -> Pos {
        Pos {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nanobot {
    pos: Pos,
    radius: i64,
}

impl FromStr for Nanobot {
    type Err = Box<::std::error::Error>;

    // pos=<-66538252,24214519,54774103>, r=94247941
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
        }
        let cap = RE.captures(s).unwrap();

        Ok(Nanobot {
            pos: Pos {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            },
            radius: cap[4].parse().unwrap(),
        })
    }
}

pub fn solve1(nanobots: Vec<Nanobot>) -> usize {
    let strongest = nanobots.iter().max_by_key(|n| n.radius).unwrap();

    let in_range = nanobots
        .iter()
        .filter(|n| strongest.pos.manhattan_distance(&n.pos) <= strongest.radius)
        .count();

    in_range
}

pub fn solve2(nanobots: Vec<Nanobot>) -> i64 {
    let mut scale = 2_i64.pow(24);
    let mut min_x = i64::max_value();
    let mut max_x = i64::min_value();
    let mut min_y = i64::max_value();
    let mut max_y = i64::min_value();
    let mut min_z = i64::max_value();
    let mut max_z = i64::min_value();

    let mut scaled_bots: Vec<Nanobot> = nanobots
        .iter()
        .map(|b| {
            let mut new = b.clone();
            new.pos = b.pos.scaled(scale);
            new.radius = b.radius / scale;
            new
        })
        .collect();

    // start with minimum area that spans all the bots.
    for bot in scaled_bots.iter() {
        if bot.pos.x < min_x {
            min_x = bot.pos.x;
        }
        if bot.pos.x > max_x {
            max_x = bot.pos.x;
        }
        if bot.pos.y < min_y {
            min_y = bot.pos.y;
        }
        if bot.pos.y > max_y {
            max_y = bot.pos.y;
        }
        if bot.pos.z < min_z {
            min_z = bot.pos.z;
        }
        if bot.pos.z > max_z {
            max_z = bot.pos.z;
        }
    }

    // Start with coarse scale, find best position, update scale and inspect area of previous position.
    // Repeat until scale is 1.
    loop {
        let xr = min_x..=max_x;
        let yr = min_y..=max_y;
        let zr = min_z..=max_z;
        let mut max_in_range = 0;
        let mut equal: Vec<Pos> = vec![];

        for z in zr {
            for y in yr.clone() {
                for x in xr.clone() {
                    let pos = Pos::new(x, y, z);
                    let count = scaled_bots
                        .iter()
                        .filter(|n| pos.manhattan_distance(&n.pos) <= n.radius)
                        .count();
                    if count > max_in_range {
                        max_in_range = count;
                        equal.clear();
                        equal.push(pos);
                    } else if count == max_in_range {
                        equal.push(pos);
                    }
                }
            }
        }

        let start = Pos::new(0, 0, 0);
        let (best, dist) = equal
            .iter()
            .map(|p| (p, p.manhattan_distance(&start)))
            .min_by_key(|&(_, dist)| dist)
            .unwrap();

        println!("max_in_range {:?}, equal: {:?}", max_in_range, equal.len());
        println!("scale: {}, best: {:?}", scale, best);

        if scale == 1 {
            return dist;
        }

        let mult = 2;
        scale = scale / mult;
        min_x = best.x * mult - 5;
        max_x = best.x * mult + 5;
        min_y = best.y * mult - 5;
        max_y = best.y * mult + 5;
        min_z = best.z * mult - 5;
        max_z = best.z * mult + 5;

        scaled_bots = nanobots
            .iter()
            .map(|b| {
                let mut new = b.clone();
                new.pos = b.pos.scaled(scale);
                new.radius = b.radius / scale;
                new
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input: Vec<Nanobot> = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        assert_eq!(solve1(input), 7);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<Nanobot> = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        assert_eq!(solve2(input), 36);
    }
}
