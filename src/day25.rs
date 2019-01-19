use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }

    fn in_constellation(&self, other: &Pos) -> bool {
        self.manhattan_distance(other) <= 3
    }
}

impl FromStr for Pos {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        }
        println!("{}", s);
        let cap = RE.captures(s.trim()).unwrap();

        Ok(Pos {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
            z: cap[3].parse()?,
            w: cap[4].parse()?,
        })
    }
}

pub fn solve1(mut points: Vec<Pos>) -> usize {
    let mut constellations: Vec<Vec<Pos>> = vec![];

    while let Some(point) = points.pop() {
        let mut constellation = vec![point];
        while let Some(idx) = points
            .iter()
            .position(|p| constellation.iter().any(|c| c.in_constellation(p)))
        {
            let p = points.remove(idx);
            constellation.push(p);
        }
        constellations.push(constellation);
    }

    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let input = r"0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        assert_eq!(
            solve1(input.trim().lines().map(|l| l.parse().unwrap()).collect()),
            2
        );
    }

    #[test]
    fn test_solve1_2() {
        let input = r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(
            solve1(input.trim().lines().map(|l| l.parse().unwrap()).collect()),
            4
        );
    }

    #[test]
    fn test_solve1_3() {
        let input = r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(
            solve1(input.trim().lines().map(|l| l.parse().unwrap()).collect()),
            3
        );
    }

    #[test]
    fn test_solve1_4() {
        let input = r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(
            solve1(input.trim().lines().map(|l| l.parse().unwrap()).collect()),
            8
        );
    }
}
