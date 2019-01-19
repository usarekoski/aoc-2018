use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Clay,
}

struct Map {
    m: Vec<Vec<Tile>>,
    visited: HashMap<Pos, Water>,
}

#[derive(Debug, Clone)]
pub struct Vein {
    x: Range<usize>,
    y: Range<usize>,
}

impl FromStr for Vein {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_X: Regex = Regex::new(r"x=(\d+)(?:\.\.(\d+))?").unwrap();
            static ref RE_Y: Regex = Regex::new(r"y=(\d+)(?:\.\.(\d+))?").unwrap();
        }
        let cap_x = RE_X.captures(s).unwrap();
        let cap_y = RE_Y.captures(s).unwrap();
        let x_start: usize = cap_x[1].parse().unwrap();
        let y_start: usize = cap_y[1].parse().unwrap();
        let x_end: usize = if let Some(end) = cap_x.get(2) {
            end.as_str().parse().unwrap()
        } else {
            x_start
        };
        let y_end: usize = if let Some(end) = cap_y.get(2) {
            end.as_str().parse().unwrap()
        } else {
            y_start
        };

        Ok(Vein {
            x: x_start..(x_end + 1),
            y: y_start..(y_end + 1),
        })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.m.iter().enumerate() {
            for (x, tile) in row.iter().enumerate().skip(430).take(150) {
                let pos = Pos::new(x, y);
                let c = match self.visited.get(&pos) {
                    Some(w) => match w {
                        Water::Flow => '|',
                        Water::Still => '~',
                    },
                    None => match tile {
                        Tile::Clay => '#',
                        Tile::Sand => '.',
                    },
                };
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Water {
    Still,
    Flow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

impl Map {
    fn convert_to_still(&mut self, pos: Pos, unvisited: &mut VecDeque<Pos>) {
        let visited = &mut self.visited;
        let Pos { x, y } = pos;
        let mut can_be_still = true;
        let mut x_1 = x;
        let mut dir = true;
        let mut converted = vec![];

        while can_be_still {
            if self.m[y][x_1] == Tile::Clay {
                if dir == false {
                    break;
                }
                // Go to other direction from starting pos.
                dir = false;
                x_1 = x + 1;
                continue;
            }
            let is_flowing = match visited.get(&Pos::new(x_1, y)) {
                Some(w) => *w == Water::Flow,
                None => false,
            };
            let over_clay = self.m[y + 1][x_1] == Tile::Clay;
            let over_still = match visited.get(&Pos::new(x_1, y + 1)) {
                Some(w) => *w == Water::Still,
                None => false,
            };

            can_be_still = is_flowing && (over_clay || over_still);
            converted.push(Pos::new(x_1, y));
            if dir {
                x_1 -= 1
            } else {
                x_1 += 1
            };
        }
        if can_be_still {
            for pos in converted.iter() {
                let old = visited.insert(*pos, Water::Still);
                assert_eq!(old, Some(Water::Flow));

                // Water flowing to this needs updating.
                let mut up = *pos;
                up.y -= 1;
                if let Some(true) = visited.get(&up).map(|w| *w == Water::Flow) {
                    unvisited.push_back(up);
                }
            }
        }
    }

    fn flow(&mut self) {
        let pos = Pos { x: 500, y: 0 };
        let mut unvisited: VecDeque<Pos> = VecDeque::new();
        unvisited.push_front(pos);
        let not_unvisited = |unvisited: &VecDeque<Pos>, pos| !unvisited.iter().any(|p| *p == pos);

        while let Some(pos) = unvisited.pop_front() {
            let Pos { x, y } = pos;
            self.visited.insert(pos, Water::Flow);
            if y + 1 >= self.m.len() {
                continue;
            }
            let down_is_sand = self.m[y + 1][x] == Tile::Sand;
            let down_is_still = match self.visited.get(&Pos::new(x, y + 1)) {
                Some(w) => *w == Water::Still,
                None => false,
            };

            if down_is_sand && !down_is_still {
                let next = Pos::new(x, y + 1);
                if !self.visited.contains_key(&next) && not_unvisited(&unvisited, next) {
                    unvisited.push_back(next);
                }
            } else {
                match self.m[y][x + 1] {
                    Tile::Sand => {
                        let next = Pos::new(x + 1, y);
                        if !self.visited.contains_key(&next) && not_unvisited(&unvisited, next) {
                            unvisited.push_back(next);
                        } else {
                            self.convert_to_still(pos, &mut unvisited);
                        }
                    }
                    Tile::Clay => {
                        self.convert_to_still(pos, &mut unvisited);
                    }
                }
                match self.m[y][x - 1] {
                    Tile::Sand => {
                        let next = Pos::new(x - 1, y);
                        if !self.visited.contains_key(&next) && not_unvisited(&unvisited, next) {
                            unvisited.push_back(next);
                        } else {
                            self.convert_to_still(pos, &mut unvisited);
                        }
                    }
                    Tile::Clay => {
                        self.convert_to_still(pos, &mut unvisited);
                    }
                }
            }
        }
    }
}

pub fn solve1(veins: Vec<Vein>) -> (i64, i64) {
    let max_x = veins.iter().map(|v| v.x.end.max(v.x.start)).max().unwrap() + 1;
    let max_y = veins.iter().map(|v| v.y.end.max(v.y.start)).max().unwrap();
    let min_y = veins.iter().map(|v| v.y.end.min(v.y.start)).min().unwrap();

    let mut map = Map {
        m: vec![vec![Tile::Sand; max_x]; max_y],
        visited: HashMap::new(),
    };

    for vein in veins.iter() {
        for row in map.m[vein.y.clone()].iter_mut() {
            for t in row[vein.x.clone()].iter_mut() {
                *t = Tile::Clay;
            }
        }
    }

    map.flow();

    let mut count_all = 0;
    let mut count_still = 0;
    for (pos, w) in map.visited.iter() {
        if pos.y >= min_y && pos.y <= max_y {
            count_all += 1;
            if *w == Water::Still {
                count_still += 1;
            }
        }
    }

    (count_all, count_still)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let input = r"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        assert_eq!(
            solve1(input.trim().lines().map(|l| l.parse().unwrap()).collect()),
            (57, 29)
        );
    }
}
