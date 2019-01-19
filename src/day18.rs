use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Acre::OpenGround => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    acres: Vec<Vec<Acre>>,
}

fn parse_acre(c: char) -> Acre {
    match c {
        '.' => Acre::OpenGround,
        '|' => Acre::Trees,
        '#' => Acre::Lumberyard,
        _ => panic!("unexpected char: {}", c),
    }
}

impl FromStr for Map {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let acres = s
            .trim()
            .lines()
            .map(|l| l.trim().chars().map(parse_acre).collect())
            .collect();

        Ok(Map { acres })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.acres.iter() {
            for acre in row.iter() {
                write!(f, "{}", acre)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Map {
    fn adjacent(&self, x_0: usize, y_0: usize) -> Vec<(usize, usize)> {
        let x = x_0 as i64;
        let y = y_0 as i64;
        [
            (x + 1, y),
            (x - 1, y),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
            (x, y + 1),
            (x, y - 1),
        ]
        .iter()
        .filter(|&(x_1, y_1)| {
            *x_1 >= 0
                && *x_1 < self.acres[y_0].len() as i64
                && *y_1 >= 0
                && *y_1 < self.acres.len() as i64
        })
        .map(|&(x_1, y_1)| (x_1 as usize, y_1 as usize))
        .collect()
    }

    fn count_adjacent(&self, x: usize, y: usize, acre: Acre) -> usize {
        self.adjacent(x, y)
            .iter()
            .filter(|(x, y)| self.acres[*y][*x] == acre)
            .count()
    }

    fn next(&mut self) -> Map {
        let mut next_acres = vec![];
        for (y, row) in self.acres.iter().enumerate() {
            let mut next_row = vec![];
            for (x, &acre) in row.iter().enumerate() {
                let next: Acre = match acre {
                    Acre::OpenGround => {
                        if self.count_adjacent(x, y, Acre::Trees) >= 3 {
                            Acre::Trees
                        } else {
                            acre
                        }
                    }
                    Acre::Trees => {
                        if self.count_adjacent(x, y, Acre::Lumberyard) >= 3 {
                            Acre::Lumberyard
                        } else {
                            acre
                        }
                    }
                    Acre::Lumberyard => {
                        if self.count_adjacent(x, y, Acre::Lumberyard) >= 1
                            && self.count_adjacent(x, y, Acre::Trees) >= 1
                        {
                            acre
                        } else {
                            Acre::OpenGround
                        }
                    }
                };
                next_row.push(next);
            }
            next_acres.push(next_row);
        }

        Map { acres: next_acres }
    }
}

pub fn solve1(s: String) -> usize {
    let mut map: Map = s.parse().unwrap();

    for _ in 0..10 {
        // println!("{}", map);
        map = map.next();
    }

    let trees = map
        .acres
        .iter()
        .flatten()
        .filter(|&&a| a == Acre::Trees)
        .count();
    let lumberyards = map
        .acres
        .iter()
        .flatten()
        .filter(|&&a| a == Acre::Lumberyard)
        .count();

    trees * lumberyards
}

pub fn solve2(s: String) -> usize {
    let mut map: Map = s.parse().unwrap();
    let mut maps: Vec<Map> = vec![];

    // Find cycle.
    let prev_map = loop {
        // println!("{}", map);
        map = map.next();
        let found = maps.iter().position(|m| *m == map);
        if let Some(prev_map) = found {
            break prev_map;
        }
        maps.push(map.clone());
    };

    let cycle = maps.len() - prev_map;
    // Corresponding index in prev_map..(prev_map + cycle).
    let cor = (1000000000 - prev_map) % cycle + prev_map - 1;

    println!("cycle: {} cor: {}", cycle, cor);

    let trees = maps[cor]
        .acres
        .iter()
        .flatten()
        .filter(|&&a| a == Acre::Trees)
        .count();
    let lumberyards = maps[cor]
        .acres
        .iter()
        .flatten()
        .filter(|&&a| a == Acre::Lumberyard)
        .count();

    trees * lumberyards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let input = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        assert_eq!(solve1(input.to_string()), 1147);
    }
}
