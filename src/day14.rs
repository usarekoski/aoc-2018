use std::fmt;

fn to_digits(mut x: i64) -> Vec<i64> {
    if x == 0 {
        return vec![0];
    }
    let mut digits = vec![];
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.reverse();
    digits
}

#[derive(Debug)]
struct Recipes {
    recipes: Vec<i64>,
    elf1: usize,
    elf2: usize,
}

impl Recipes {
    fn add_recipes(&mut self) {
        let sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        for digit in to_digits(sum) {
            self.recipes.push(digit);
        }
    }

    fn select_recipes(&mut self) {
        let length = self.recipes.len();
        self.elf1 = ((self.elf1 + self.recipes[self.elf1] as usize + 1) % length) as usize;
        self.elf2 = ((self.elf2 + self.recipes[self.elf2] as usize + 1) % length) as usize;
    }
}

impl fmt::Display for Recipes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, d) in self.recipes.iter().enumerate() {
            if i == self.elf1 {
                write!(f, "({}) ", d)?
            } else if i == self.elf2 {
                write!(f, "[{}] ", d)?
            } else {
                write!(f, "{} ", d)?
            }
        }
        Ok(())
    }
}

pub fn solve1(n_receipes: usize) -> String {
    let mut recipes = Recipes {
        recipes: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };
    for _ in 0..(n_receipes + 10) {
        recipes.add_recipes();
        recipes.select_recipes();
        // println!("{}", recipes);
    }
    let last_10 = recipes.recipes[(n_receipes)..(n_receipes + 10)]
        .iter()
        .map(|d| d.to_string())
        .collect();
    last_10
}

pub fn solve2(scores: &str) -> usize {
    let mut recipes = Recipes {
        recipes: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };
    let scores: Vec<i64> = scores
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i64)
        .collect();
    let len = scores.len();

    loop {
        recipes.add_recipes();
        recipes.select_recipes();

        let r_len = recipes.recipes.len();
        // One or two receipes are added, so compare last scores and one before that.
        let range1 = r_len.saturating_sub(len)..;
        let range2 = r_len.saturating_sub(len + 1)..(r_len - 1);
        if *scores.as_slice() == recipes.recipes[range1] {
            return r_len - len;
        } else if *scores.as_slice() == recipes.recipes[range2] {
            return r_len - len - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(123), vec![1, 2, 3]);
        assert_eq!(to_digits(901), vec![9, 0, 1]);
        assert_eq!(to_digits(5), vec![5]);
        assert_eq!(to_digits(0), vec![0]);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(9), "5158916779");
        assert_eq!(solve1(5), "0124515891");
        assert_eq!(solve1(18), "9251071085");
        assert_eq!(solve1(2018), "5941429882");
    }

    #[test]
    fn test2() {
        assert_eq!(solve2("51589"), 9);
        assert_eq!(solve2("01245"), 5);
        assert_eq!(solve2("92510"), 18);
        assert_eq!(solve2("59414"), 2018);
    }
}
