use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
pub struct Claim {
    id: i64,
    // left: i64,
    // top: i64,
    // width: i64,
    // height: i64,
    areas: HashSet<(i64, i64)>,
}

impl FromStr for Claim {
    type Err = Box<::std::error::Error>;

    // #1273 @ 134,911: 13x12
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let cap = RE.captures(s).unwrap();

        let left: i64 = cap[2].parse()?;
        let top: i64 = cap[3].parse()?;
        let width: i64 = cap[4].parse()?;
        let height: i64 = cap[5].parse()?;

        let mut areas = HashSet::new();
        for x in (left + 1)..=(left + width) {
            for y in (top + 1)..=(top + height) {
                areas.insert((x, y));
            }
        }

        Ok(Claim {
            id: cap[1].parse()?,
            // left: cap[2].parse()?,
            // top: cap[3].parse()?,
            // width: cap[4].parse()?,
            // height: cap[5].parse()?,
            areas: areas,
        })
    }
}

pub fn solve1(claims: Vec<Claim>) -> i64 {
    let mut overlapping = HashSet::new();

    for (pos, claim) in claims.iter().enumerate() {
        for other_claim in claims[(pos + 1)..].iter() {
            for overlap in claim.areas.intersection(&other_claim.areas) {
                overlapping.insert(overlap);
            }
        }
    }

    overlapping.len() as i64
}

pub fn solve2(claims: Vec<Claim>) -> i64 {
    let mut overlapping: HashSet<(i64, i64)> = HashSet::new();

    for (pos, claim) in claims.iter().enumerate() {
        for other_claim in claims[(pos + 1)..].iter() {
            for overlap in claim.areas.intersection(&other_claim.areas) {
                overlapping.insert(*overlap);
            }
        }
    }

    for claim in claims.iter() {
        if claim.areas.is_disjoint(&overlapping) {
            return claim.id;
        }
    }

    panic!("no solution found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ids = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .iter()
            .map(|&s| String::from(s).parse::<Claim>().unwrap())
            .collect();

        assert_eq!(solve1(ids), 4);
    }

    #[test]
    fn test2() {
        let ids = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .iter()
            .map(|&s| String::from(s).parse::<Claim>().unwrap())
            .collect();

        assert_eq!(solve2(ids), 3);
    }
}
