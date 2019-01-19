use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum UnitType {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Unit {
    pos: Pos,
    class: UnitType,
    hp: i64,
    attack: i64,
    removed: bool,
}

impl Unit {
    fn new(class: UnitType, pos: Pos) -> Unit {
        Unit {
            class: class,
            hp: 200,
            pos: pos,
            attack: 3,
            removed: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { y, x }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    units: Vec<Unit>,
    end_on_elf_death: bool,
}

fn parse_tile(c: char, pos: Pos) -> (Tile, Option<Unit>) {
    match c {
        '#' => (Tile::Wall, None),
        '.' => (Tile::Open, None),
        'G' => (Tile::Open, Some(Unit::new(UnitType::Goblin, pos))),
        'E' => (Tile::Open, Some(Unit::new(UnitType::Elf, pos))),
        _ => panic!("unexpected char: {}", c),
    }
}

fn tile_to_char(t: &Tile) -> char {
    match t {
        Tile::Wall => '#',
        Tile::Open => '.',
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EndReason {
    ElfDied,
    CombatEnds,
    Continue,
}

impl FromStr for Map {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initial = s.lines().enumerate().map(|(y, l)| {
            l.trim_end_matches('\n')
                .chars()
                .enumerate()
                .map(move |(x, c)| parse_tile(c, Pos::new(x, y)))
        });

        let units = initial
            .clone()
            .flatten()
            .map(|(_, u)| u)
            .flatten()
            .collect();
        let tiles = initial.map(|l| l.map(|(t, _)| t).collect()).collect();
        Ok(Map {
            tiles,
            units,
            end_on_elf_death: false,
        })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(unit) = self
                    .units
                    .iter()
                    .find(|u| u.removed == false && u.pos == Pos::new(x, y))
                {
                    let c = match unit.class {
                        UnitType::Goblin => 'G',
                        UnitType::Elf => 'E',
                    };
                    write!(f, "{}", c)?
                } else {
                    write!(f, "{}", tile_to_char(tile))?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Map {
    fn adjacent(&self, pos: Pos) -> Vec<Pos> {
        let Pos { x, y } = pos;
        let mut adj = vec![];
        if x + 1 < self.tiles[y].len() {
            adj.push(Pos::new(x + 1, y));
        }
        if y + 1 < self.tiles.len() {
            adj.push(Pos::new(x, y + 1));
        }
        if x >= 1 {
            adj.push(Pos::new(x - 1, y));
        }
        if y >= 1 {
            adj.push(Pos::new(x, y - 1));
        }
        adj
    }

    fn get_tile(&self, pos: Pos) -> &Tile {
        let Pos { x, y } = pos;
        &self.tiles[y][x]
    }

    // distance with last step before reaching target.
    fn find_target(&self, unit: Pos, enemy_pos: Pos) -> Option<(i64, Pos)> {
        let mut dist: HashMap<Pos, i64> = HashMap::new();
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut unvisited: VecDeque<Pos> = VecDeque::new();
        let mut last_steps = vec![];
        unvisited.push_back(unit);
        dist.insert(unit, 0);

        while let Some(next) = unvisited.pop_front() {
            if visited.contains(&next) {
                continue;
            }
            let d = *dist.get(&next).unwrap();
            for a in self.adjacent(next).iter().filter(|&&a| {
                let has_unit = self
                    .units
                    .iter()
                    .filter(|u| u.removed == false)
                    .find(|u| u.pos == a && u.pos != enemy_pos)
                    .is_some();
                *self.get_tile(a) != Tile::Wall && !has_unit
            }) {
                let cur_dist = dist.entry(*a).or_insert(i64::max_value());
                if *cur_dist >= d + 1 {
                    *cur_dist = d + 1;
                    if *a == enemy_pos {
                        last_steps.push((*cur_dist, next));
                    }
                }
                unvisited.push_back(*a);
            }
            visited.insert(next);
        }

        // Sorting order: first distance, then last step's reading order.
        last_steps.sort();
        dist.get(&enemy_pos)
            .map(|v| (*v, last_steps.get(0).unwrap().1))
    }

    fn find_adjacent_enemy(&self, idx: usize) -> Option<usize> {
        let unit = &self.units[idx];
        let mut enemies: Vec<_> = self
            .adjacent(unit.pos)
            .iter()
            .map(|a| {
                self.units
                    .iter()
                    .enumerate()
                    .find(|(_idx, u)| u.removed == false && u.class != unit.class && u.pos == *a)
                    .map(|(idx, u)| (u.hp, *a, idx))
            })
            .flatten()
            .collect();

        // Sorting order: lowest hp first, then tile reading order.
        enemies.sort();
        enemies.get(0).map(|(_hp, _pos, idx)| *idx)
    }

    fn advance_turn(&mut self) -> EndReason {
        self.units.sort();
        for i in 0..self.units.len() {
            if self.units[i].removed {
                continue;
            }
            let class = self.units[i].class;
            let pos = self.units[i].pos;
            let attack = self.units[i].attack;
            if self.find_adjacent_enemy(i).is_none() {
                // Move
                let enemies: Vec<&Unit> = self
                    .units
                    .iter()
                    .filter(|e| e.class != class && e.removed == false)
                    .collect();
                // No enemies found, combat ends.
                if enemies.len() == 0 {
                    return EndReason::CombatEnds;
                }
                let mut reachable_enemies: Vec<_> = enemies
                    .iter()
                    .map(|e| {
                        self.find_target(e.pos, pos).map(|(dist, first_step)| {
                            let (dist2, last_step) = self.find_target(pos, e.pos).unwrap();
                            assert_eq!(dist, dist2);
                            // For sorting order.
                            (dist, last_step, first_step)
                        })
                    })
                    .flatten()
                    .collect();

                // Distance first, then last step's reading order
                // and lastly first step's reading order.
                reachable_enemies.sort();
                if let Some((_dist, _, first_step)) = reachable_enemies.get(0) {
                    self.units[i].pos = *first_step;
                }
            }

            // attack
            if let Some(enemy_idx) = self.find_adjacent_enemy(i) {
                let enemy = &mut self.units[enemy_idx];
                enemy.hp -= attack;
                if enemy.hp <= 0 {
                    enemy.removed = true;
                    if self.end_on_elf_death && enemy.class == UnitType::Elf {
                        return EndReason::ElfDied;
                    }
                }
            }
        }

        EndReason::Continue
    }
}

pub fn solve1(s: String) -> i64 {
    let mut map: Map = s.parse().unwrap();
    let mut turns: i64 = 0;

    while map.advance_turn() == EndReason::Continue {
        turns += 1;
    }

    let hit_points: i64 = map
        .units
        .iter()
        .filter(|u| u.removed == false)
        .map(|u| u.hp as i64)
        .sum();
    println!("turns: {}, hp: {}", turns, hit_points);

    turns * hit_points
}

pub fn solve2(s: String) -> i64 {
    let mut map: Map = s.parse().unwrap();
    map.end_on_elf_death = true;
    let units_on_start = map.units.clone();
    let mut reason = EndReason::Continue;
    let mut turns: i64 = 0;
    let mut iterations = 1;
    while reason != EndReason::CombatEnds {
        turns = 0;
        reason = EndReason::Continue;
        map.units = units_on_start
            .iter()
            .map(|u| {
                let mut new = u.clone();
                if new.class == UnitType::Elf {
                    new.attack += iterations;
                }
                new
            })
            .collect();

        while reason == EndReason::Continue {
            reason = map.advance_turn();
            turns += 1;
        }
        // Go back to last full round.
        turns -= 1;
        println!("attack: {} ", 3 + iterations);
        iterations += 1;
    }

    let hit_points: i64 = map
        .units
        .iter()
        .filter(|u| u.removed == false)
        .map(|u| u.hp as i64)
        .sum();

    println!("turns: {}, hp: {}", turns, hit_points);
    turns * hit_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(solve1(input.to_string()), 27730);
    }

    #[test]
    fn test_solve1_2() {
        let input = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        assert_eq!(solve1(input.to_string()), 36334);
    }

    #[test]
    fn test_solve1_3() {
        let input = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(solve1(input.to_string()), 39514);
    }

    #[test]
    fn test_solve1_4() {
        let input = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(solve1(input.to_string()), 27755);
    }

    #[test]
    fn test_solve1_5() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(solve1(input.to_string()), 18740);
    }

    #[test]
    fn test_solve2_1() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(solve2(input.to_string()), 4988);
    }

    #[test]
    fn test_solve2_2() {
        let input = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(solve2(input.to_string()), 31284);
    }

    #[test]
    fn test_solve2_4() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(solve2(input.to_string()), 1140);
    }
}
