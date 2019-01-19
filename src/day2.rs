use std::collections::HashSet;

// contains 2 and/or 3 same characters.
// Returns (contains_two_same, contains_three_same).
fn contains_n_same(s: &str) -> (bool, bool) {
    let mut contains = (false, false);
    let mut seen = HashSet::new();

    for (pos, c) in s.char_indices() {
        if seen.contains(&c) {
            continue;
        }

        let count = s[pos..].chars().filter(|&v| v == c).count();
        match count {
            2 => contains.0 = true,
            3 => contains.1 = true,
            _ => (),
        }
        seen.insert(c);
    }

    contains
}

pub fn solve1(ids: Vec<String>) -> i64 {
    let (two_same, three_same): (i64, i64) = ids
        .iter()
        .map(|id| contains_n_same(id))
        .fold((0, 0), |(two_sum, three_sum), (two, three)| {
            (two_sum + two as i64, three_sum + three as i64)
        });

    two_same * three_same
}

// Do strings differ by one character.
fn differ_by_one(a: &str, b: &str) -> bool {
    let mut differ = false;

    for (c_a, c_b) in a.chars().zip(b.chars()) {
        if c_a != c_b {
            if !differ {
                differ = true;
            } else {
                return false;
            }
        }
    }

    differ
}

pub fn solve2(ids: Vec<String>) -> String {
    for (pos, id) in ids.iter().enumerate() {
        if let Some(other_id) = ids[pos..].iter().find(|v| differ_by_one(v, id)) {
            let mut common = String::new();
            for (c_a, c_b) in id.chars().zip(other_id.chars()) {
                if c_a == c_b {
                    common.push(c_a);
                }
            }
            return common;
        }
    }

    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ids = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
            .iter()
            .map(|&s| String::from(s))
            .collect();

        assert_eq!(solve1(ids), 12);
    }

    #[test]
    fn test2() {
        let ids = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
            .iter()
            .map(|&s| String::from(s))
            .collect();

        assert_eq!(solve2(ids), "fgij");
    }
}
