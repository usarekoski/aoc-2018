use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Pos {
        Pos { x, y }
    }
}

fn adjacent(pos: Pos) -> Vec<Pos> {
    let Pos { x, y } = pos;
    vec![
        Pos::new(x + 1, y),
        Pos::new(x, y + 1),
        Pos::new(x - 1, y),
        Pos::new(x, y - 1),
    ]
}

fn distances(doors: &HashSet<(Pos, Pos)>) -> HashMap<Pos, i64> {
    let mut dist: HashMap<Pos, i64> = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut unvisited: VecDeque<Pos> = VecDeque::new();
    let start = Pos::new(0, 0);
    unvisited.push_back(start);
    dist.insert(start, 0);

    while let Some(next) = unvisited.pop_front() {
        if visited.contains(&next) {
            continue;
        }
        let d = *dist.get(&next).unwrap();
        for a in adjacent(next).iter().filter(|&&a| {
            let has_door = doors.contains(&(next, a)) || doors.contains(&(a, next));
            has_door
        }) {
            let cur_dist = dist.entry(*a).or_insert(i64::max_value());
            if *cur_dist >= d + 1 {
                *cur_dist = d + 1;
            }
            unvisited.push_back(*a);
        }
        visited.insert(next);
    }

    dist
}

fn parse_doors(s: String) -> HashSet<(Pos, Pos)> {
    let mut doors: HashSet<(Pos, Pos)> = HashSet::new();
    let mut branches: Vec<Pos> = vec![];
    let mut x = 0;
    let mut y = 0;

    for c in s.trim().chars() {
        match c {
            '^' => (),
            '$' => (),
            'N' => {
                doors.insert((Pos::new(x, y), Pos::new(x, y + 1)));
                y += 1;
            }
            'S' => {
                doors.insert((Pos::new(x, y), Pos::new(x, y - 1)));
                y -= 1;
            }
            'E' => {
                doors.insert((Pos::new(x, y), Pos::new(x + 1, y)));
                x += 1;
            }
            'W' => {
                doors.insert((Pos::new(x, y), Pos::new(x - 1, y)));
                x -= 1;
            }
            '(' => {
                branches.push(Pos::new(x, y));
            }
            '|' => {
                let pos = branches[branches.len() - 1];
                x = pos.x;
                y = pos.y;
            }
            ')' => {
                let r = branches.pop();
                assert!(r.is_some());
            }
            _ => panic!("unexpected char {}", c),
        }
    }
    assert_eq!(branches.len(), 0);
    doors
}

pub fn solve1(s: String) -> i64 {
    let doors = parse_doors(s);
    let dist = distances(&doors);
    dist.values().max().map(|m| *m).unwrap()
}

pub fn solve2(s: String) -> i64 {
    let doors = parse_doors(s);
    let dist = distances(&doors);
    dist.values().filter(|&&distance| distance >= 1000).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let input = r"^WNE$";
        assert_eq!(solve1(input.to_string()), 3);
    }

    #[test]
    fn test_solve1_2() {
        let input = r"^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(solve1(input.to_string()), 10);
    }

    #[test]
    fn test_solve1_3() {
        let input = r"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(solve1(input.to_string()), 18);
    }

    #[test]
    fn test_solve1_4() {
        let input = r"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(solve1(input.to_string()), 23);
    }

    #[test]
    fn test_solve1_5() {
        let input = r"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(solve1(input.to_string()), 31);
    }
}
