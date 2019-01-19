use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    rule: Vec<bool>,
    result: bool,
}

#[derive(Debug)]
pub struct Pots {
    generations: Vec<Vec<bool>>,
    rules: Vec<Rule>,
}

fn parse_plant(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("unexpected char"),
    }
}

const OFFSET: usize = 500;

impl FromStr for Pots {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut initial: Vec<bool> = s
            .lines()
            .nth(0)
            .and_then(|l| l.split("state: ").nth(1))
            .map(|l| l.trim().chars().map(parse_plant).collect())
            .unwrap();

        let rules = s
            .lines()
            .skip(2)
            .map(|l| {
                let mut it = l.trim().split(" => ");
                let rule = it.next().unwrap().chars().map(parse_plant).collect();
                let result = it.next().unwrap().chars().map(parse_plant).nth(0).unwrap();
                Rule {
                    rule: rule,
                    result: result,
                }
            })
            .collect();

        let mut pots = vec![false; OFFSET];
        pots.append(&mut initial);
        pots.append(&mut vec![false; OFFSET]);
        Ok(Pots {
            generations: vec![pots],
            rules,
        })
    }
}

impl Pots {
    fn will_have_plant(&self, pos: usize) -> bool {
        let start = pos - 2;
        let stop = pos + 2;
        let prev = &self.generations[self.generations.len() - 1];
        let range: Vec<bool> = (start..=stop)
            .map(|i| *prev.get(i).unwrap_or(&false))
            .collect();

        assert!(range.len() == 5);
        match self.rules.iter().find(|r| r.rule == range) {
            Some(rule) => rule.result,
            None => false,
        }
    }

    fn next_generation(&mut self) {
        let next = self.generations[self.generations.len() - 1]
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                if idx < 2 {
                    *p
                } else {
                    self.will_have_plant(idx)
                }
            })
            .collect();
        self.generations.push(next);
    }

    #[allow(dead_code)]
    fn print_pots(&self) {
        println!(
            "{} {}",
            self.generations[self.generations.len() - 1]
                .iter()
                .map(|p| match p {
                    true => '#',
                    false => '.',
                })
                .collect::<String>(),
            self.sum_pots(self.generations.len() - 1)
        );
    }

    fn matches(a: &Vec<bool>, b: &Vec<bool>) -> bool {
        let a_start = a.iter().skip_while(|&&x| x == false);
        let b_start = b.iter().skip_while(|&&x| x == false);

        a_start.zip(b_start).all(|(&x, &y)| x == y)
    }

    fn matches_previous_generation(&self) -> Option<usize> {
        let last_idx = self.generations.len() - 1;
        let last = &self.generations[last_idx];
        self.generations
            .iter()
            .enumerate()
            .take(last_idx)
            .find(|(_, g)| Pots::matches(last, g))
            .map(|(i, _)| i)
    }

    fn sum_pots(&self, gen: usize) -> i64 {
        self.generations[gen]
            .iter()
            .enumerate()
            .filter(|(_, &p)| p)
            .map(|(i, _)| i as i64 - OFFSET as i64)
            .sum()
    }
}

pub fn solve1(s: String) -> i64 {
    let mut pots: Pots = s.parse().unwrap();
    // pots.print_pots();
    for _ in 0..20 {
        pots.next_generation();
        // pots.print_pots();
    }

    pots.sum_pots(pots.generations.len() - 1)
}

pub fn solve2(s: String) -> usize {
    let mut pots: Pots = s.parse().unwrap();
    let mut gen = 0;
    let matching_generation = loop {
        pots.next_generation();
        gen += 1;
        match pots.matches_previous_generation() {
            Some(v) => break v,
            None => (),
        }
    };

    // Pot pattern repeats, but moves to right, increasing sum by diff every generation.
    let rounds = 50000000000usize;
    let sum = pots.sum_pots(gen) as usize;
    let prev_sum = pots.sum_pots(matching_generation) as usize;
    let diff = sum - prev_sum;
    (rounds - gen) * diff + sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

        assert_eq!(solve1(input.to_string()), 325);
    }
}
