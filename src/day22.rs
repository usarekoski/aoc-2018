use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            RegionType::Rocky => '.',
            RegionType::Wet => '=',
            RegionType::Narrow => '|',
        };
        write!(f, "{}", c)
    }
}

struct Cave {
    target: Pos,
    depth: usize,
    geologic_index: HashMap<Pos, usize>,
}

impl Cave {
    fn geologic_index(&mut self, pos: Pos) -> usize {
        match self.geologic_index.get(&pos) {
            Some(idx) => *idx,
            None => {
                let idx = match pos {
                    Pos { x: 0, y: 0 } => 0,
                    _ if pos == self.target => 0,
                    Pos { y: 0, x } => x * 16807,
                    Pos { y, x: 0 } => y * 48271,
                    Pos { y, x } => {
                        self.erosion_level(Pos::new(x - 1, y))
                            * self.erosion_level(Pos::new(x, y - 1))
                    }
                };
                self.geologic_index.insert(pos, idx);
                idx
            }
        }
    }

    fn erosion_level(&mut self, pos: Pos) -> usize {
        (self.geologic_index(pos) + self.depth) % 20183
    }

    fn region_type(&mut self, pos: Pos) -> RegionType {
        match self.erosion_level(pos) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }

    fn risk_level(&mut self, pos: Pos) -> usize {
        match self.region_type(pos) {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }

    fn area_risk_level(&mut self) -> usize {
        let mut sum = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                sum += self.risk_level(Pos::new(x, y));
            }
        }

        sum
    }

    // up, down, left, right and tool change.
    fn adjacent(&mut self, pos: PosT) -> Vec<(PosT, i64)> {
        let PosT { x, y, tool } = pos;
        let x_1 = x as i64;
        let y_1 = y as i64;
        let mut adj: Vec<(PosT, i64)> = [
            (x_1 + 1, y_1),
            (x_1, y_1 + 1),
            (x_1 - 1, y_1),
            (x_1, y_1 - 1),
        ]
        .iter()
        .filter(|&&(x_2, y_2)| x_2 >= 0 && y_2 >= 0)
        .map(|&(x_2, y_2)| {
            let region = self.region_type(Pos::new(x_2 as usize, y_2 as usize));
            if tool.is_valid_tool(region) {
                Some((PosT::new(x_2 as usize, y_2 as usize, tool), 1))
            } else {
                None
            }
        })
        .flatten()
        .collect();

        match tool.change_tool(self.region_type(pos.pos())) {
            Some(changed_tool) => adj.push((PosT::new(x, y, changed_tool), 7)),
            None => panic!("unexpected tool {:?}", tool),
        }

        adj
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tool {
    fn change_tool(&self, region: RegionType) -> Option<Tool> {
        match region {
            RegionType::Rocky => match self {
                Tool::ClimbingGear => Some(Tool::Torch),
                Tool::Torch => Some(Tool::ClimbingGear),
                _ => None,
            },
            RegionType::Wet => match self {
                Tool::ClimbingGear => Some(Tool::Neither),
                Tool::Neither => Some(Tool::ClimbingGear),
                _ => None,
            },
            RegionType::Narrow => match self {
                Tool::Torch => Some(Tool::Neither),
                Tool::Neither => Some(Tool::Torch),
                _ => None,
            },
        }
    }

    fn is_valid_tool(&self, region: RegionType) -> bool {
        self.change_tool(region).is_some()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PosT {
    y: usize,
    x: usize,
    tool: Tool,
}

impl PosT {
    fn new(x: usize, y: usize, tool: Tool) -> PosT {
        PosT { x, y, tool }
    }

    fn pos(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x,
        }
    }
}

// Use Dijkstra's shortest path algorithm for finding shortest durations.
// Use std::cmp::Reverse to change BinaryHeap to min-heap.
fn duration(cave: &mut Cave, start: PosT, target: PosT) -> HashMap<PosT, i64> {
    let mut visited: HashMap<PosT, i64> = HashMap::new();
    let mut unvisited: BinaryHeap<Reverse<(i64, PosT)>> = BinaryHeap::new();
    unvisited.push(Reverse((0, start)));

    while let Some(n) = unvisited.pop() {
        let (d, next) = n.0;
        if visited.contains_key(&next) {
            continue;
        }
        for (adj, adj_dist) in cave.adjacent(next).iter() {
            unvisited.push(Reverse((d + adj_dist, *adj)));
        }
        visited.insert(next, d);

        if next == target {
            break;
        }
    }

    visited
}

pub fn solve1(depth: usize, target_x: usize, target_y: usize) -> usize {
    let mut cave = Cave {
        depth,
        target: Pos::new(target_x, target_y),
        geologic_index: HashMap::new(),
    };

    cave.area_risk_level()
}

pub fn solve2(depth: usize, target_x: usize, target_y: usize) -> i64 {
    let mut cave = Cave {
        depth,
        target: Pos::new(target_x, target_y),
        geologic_index: HashMap::new(),
    };

    let start = PosT::new(0, 0, Tool::Torch);
    let target = PosT::new(target_x, target_y, Tool::Torch);
    let durations = duration(&mut cave, start, target);
    let duration = durations.get(&target).unwrap();

    *duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(510, 10, 10), 114);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(510, 10, 10), 45);
    }
}
