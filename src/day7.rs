use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Step {
    id: u32,
}

impl Step {
    fn name(&self) -> char {
        std::char::from_u32(self.id + 64).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    before: Step,
    after: Step,
}

impl FromStr for Dependency {
    type Err = Box<::std::error::Error>;

    // input: "1, 2"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
        }
        let cap = RE.captures(s).unwrap();

        Ok(Dependency {
            before: Step {
                id: u32::from(cap[1].chars().nth(0).unwrap()) - 64,
            },
            after: Step {
                id: u32::from(cap[2].chars().nth(0).unwrap()) - 64,
            },
        })
    }
}

// Find next step to take, steps are ordered alphabetically,
// because ascii characters as integer values are.
fn find_next(deps: &Vec<Dependency>) -> Option<Step> {
    deps.iter()
        .filter(|d| !deps.iter().any(|d1| d1.after == d.before))
        .min_by_key(|d| d.before)
        .map(|next| next.before)
}

// Solution: JMQZELVYXTIGPHFNSOADKWBRUC
pub fn solve1(mut deps: Vec<Dependency>) -> String {
    // store all steps to get the last step in the end.
    let mut all_steps: HashSet<Step> = HashSet::new();
    for d in deps.iter() {
        all_steps.insert(d.before);
        all_steps.insert(d.after);
    }

    let mut order: Vec<Step> = vec![];

    while deps.len() > 0 {
        let next = find_next(&deps).expect("no next step found");
        order.push(next);
        all_steps.remove(&next);
        deps.retain(|d| d.before != next);
    }

    let last = all_steps.drain().next();
    order.iter().chain(last.iter()).map(|s| s.name()).collect()
}

pub fn solve2(deps: Vec<Dependency>, n_workers: usize, add_duration: u32) -> u32 {
    let mut seconds = 0;
    let mut workers: Vec<Option<(Step, u32)>> = vec![None; n_workers];
    let mut done: HashSet<Step> = HashSet::new();

    while done.is_empty() || workers.iter().any(|w| w.is_some()) {
        // Check if worker is done.
        for w in workers.iter_mut() {
            if let Some((s, start)) = w {
                if seconds >= s.id + *start + add_duration {
                    done.insert(*s);
                    *w = None;
                }
            }
        }

        // assing work
        while let Some(free_worker) = workers.iter().position(|w| w.is_none()) {
            let next = deps
                .iter()
                // before step is done and after is not.
                .filter(|d| !done.contains(&d.after))
                .filter(|d| {
                    deps.iter()
                        .filter(|d1| d1.after == d.after)
                        .all(|d1| done.contains(&d1.before))
                })
                .map(|d| d.after)
                // + all with no before dependencies.
                .chain(
                    deps.iter()
                        .filter(|d| deps.iter().all(|d1| d1.after != d.before))
                        .map(|d| d.before)
                        .filter(|d| !done.contains(d)),
                )
                // Is not worked by anyone.
                .filter(|step| !workers.iter().any(|w| w.map_or(false, |(s, _)| s == *step)))
                .min();
            if let Some(n) = next {
                workers[free_worker] = Some((n, seconds));
            } else {
                break;
            }
        }

        // println!("{:?}", workers);
        seconds += 1;
    }

    seconds - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
        .iter()
        .map(|&s| String::from(s).parse::<Dependency>().unwrap())
        .collect();

        assert_eq!(solve1(input), "CABDFE");
    }

    #[test]
    fn test2() {
        let input = [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
        .iter()
        .map(|&s| String::from(s).parse::<Dependency>().unwrap())
        .collect();

        assert_eq!(solve2(input, 2, 0), 15);
    }
}
