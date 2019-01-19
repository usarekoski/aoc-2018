use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum AttackType {
    Bludgeoning,
    Fire,
    Slashing,
    Cold,
    Radiation,
}

impl FromStr for AttackType {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bludgeoning" => AttackType::Bludgeoning,
            "fire" => AttackType::Fire,
            "slashing" => AttackType::Slashing,
            "cold" => AttackType::Cold,
            "radiation" => AttackType::Radiation,
            _ => panic!("unknown attack type: {}", s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Army {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    hit_points: i64,
    attack_damage: i64,
    attack_type: AttackType,
    initiative: i64,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    units: i64,
    army: Army,
}

impl Group {
    fn effective_power(&self) -> i64 {
        self.attack_damage * self.units
    }

    fn take_damage(&mut self, damage: i64) {
        let removed = damage / self.hit_points;
        self.units -= removed;
    }
}

impl FromStr for Group {
    type Err = Box<::std::error::Error>;

    // this parses only immmunities first, then weaknesses,
    // so if they are in opposite order in input, they must be swapped manually.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RE_S: &str = r"^(\d+) units each with (\d+) hit points(?: \((?:immune to ((?:\w|,| )+))?(?:; )?(?:weak to ((?:\w|,| )+))?\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)$";
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_S).unwrap();
        }

        let cap = RE.captures(s).unwrap();
        let attack_types = |capt: Option<regex::Match>| match capt {
            None => vec![],
            Some(s) => s.as_str().split(", ").map(|a| a.parse().unwrap()).collect(),
        };
        Ok(Group {
            units: cap[1].parse()?,
            hit_points: cap[2].parse()?,
            immunities: attack_types(cap.get(3)),
            weaknesses: attack_types(cap.get(4)),
            attack_damage: cap[5].parse()?,
            attack_type: cap[6].parse()?,
            initiative: cap[7].parse()?,
            army: Army::ImmuneSystem, // Update later.
        })
    }
}

fn damage(attacking: &Group, defending: &Group) -> i64 {
    let base = attacking.effective_power();
    let is_immune = defending
        .immunities
        .iter()
        .any(|imm| *imm == attacking.attack_type);
    if is_immune {
        return 0;
    }
    let is_weak = defending
        .weaknesses
        .iter()
        .any(|weak| *weak == attacking.attack_type);
    if is_weak {
        base * 2
    } else {
        base
    }
}

fn select_target<'a>(
    attacking: &Group,
    defending: impl Iterator<Item = (usize, &'a Group)>,
) -> Option<usize> {
    defending
        .max_by(|&(_, a), &(_, b)| {
            (damage(attacking, a), a.effective_power(), a.initiative).cmp(&(
                damage(attacking, b),
                b.effective_power(),
                b.initiative,
            ))
        })
        .and_then(|(idx, g)| {
            if damage(attacking, g) <= 0 {
                None
            } else {
                Some(idx)
            }
        })
}

fn parse_groups(s: String) -> Vec<Group> {
    let immune_system = s
        .trim()
        .lines()
        .skip(1)
        .take_while(|l| l.trim().len() > 0)
        .map(|l| l.parse().unwrap())
        .map(|mut g: Group| {
            g.army = Army::ImmuneSystem;
            g
        });

    let infection = s
        .trim()
        .lines()
        .skip_while(|l| l.trim().len() > 0)
        .skip(2)
        .map(|l| l.parse().unwrap())
        .map(|mut g: Group| {
            g.army = Army::Infection;
            g
        });

    immune_system.chain(infection).collect()
}

fn combat(groups: &mut Vec<Group>) {
    loop {
        // does combat end?
        let test_army = &groups[0].army;
        if groups.iter().all(|g| g.army == *test_army) {
            break;
        }

        // target selection
        let mut targets: HashMap<usize, Option<usize>> = HashMap::new();
        loop {
            let max_res = groups
                .iter()
                .enumerate()
                .filter(|(i, _)| !targets.contains_key(i))
                .max_by(|(_, a), (_, b)| {
                    (a.effective_power(), a.initiative).cmp(&(b.effective_power(), b.initiative))
                });
            if max_res.is_none() {
                break;
            }
            let (max_idx, max) = max_res.unwrap();

            let target = select_target(
                max,
                groups.iter().enumerate().filter(|(i, g)| {
                    g.army != max.army && !targets.values().flatten().any(|v| v == i)
                }),
            );
            targets.insert(max_idx, target);
        }

        // attacking
        let mut has_attacked: HashSet<usize> = HashSet::new();
        let mut has_dead: HashSet<usize> = HashSet::new();
        loop {
            let max_res = groups
                .iter()
                .enumerate()
                .filter(|(i, _)| !has_attacked.contains(i))
                .max_by(|(_, a), (_, b)| a.initiative.cmp(&b.initiative));
            if max_res.is_none() {
                break;
            }
            let (max_idx, max) = max_res.unwrap();
            if max.units <= 0 {
                has_attacked.insert(max_idx);
                continue;
            }
            let target_idx = {
                match targets.get(&max_idx) {
                    None | Some(None) => {
                        has_attacked.insert(max_idx);
                        continue;
                    }
                    Some(Some(v)) => *v,
                }
            };
            let target = &groups[target_idx];
            let damage = damage(max, target);
            groups[target_idx].take_damage(damage);
            if groups[target_idx].units <= 0 {
                has_dead.insert(target_idx);
            }
            has_attacked.insert(max_idx);
        }
        let mut new_groups: Vec<Group> = groups
            .iter()
            .enumerate()
            .filter(|(i, _)| !has_dead.contains(&i))
            .map(|(_, g)| g.clone())
            .collect();
        std::mem::swap(&mut new_groups, groups);
    }
}

pub fn solve1(s: String) -> i64 {
    let mut groups: Vec<Group> = parse_groups(s);
    combat(&mut groups);
    groups.iter().map(|g| g.units).sum()
}

pub fn solve2(s: String) -> i64 {
    let groups: Vec<Group> = parse_groups(s);
    // 0-34 infection wins, 35 seems to be stalemate (runs forever), so skip to 36.
    let mut boost = 36;
    loop {
        println!("boost {}", boost);
        let mut boosted: Vec<Group> = groups
            .iter()
            .map(|g| {
                let mut new_g: Group = g.clone();
                if new_g.army == Army::ImmuneSystem {
                    new_g.attack_damage += boost;
                }
                new_g
            })
            .collect();
        combat(&mut boosted);
        if boosted.iter().all(|g| g.army == Army::ImmuneSystem) {
            return boosted.iter().map(|g| g.units).sum();
        }
        boost += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
            .to_string();

        assert_eq!(solve1(input), 5216);
    }

    #[test]
    fn test_solve2() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
            .to_string();

        assert_eq!(solve2(input), 51);
    }
}
