use std::collections::HashSet;

pub fn solve1(frequencies: Vec<i64>) -> i64 {
    frequencies.iter().sum()
}

pub fn solve2(frequencies: Vec<i64>) -> i64 {
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut sum = 0;

    for fr in frequencies.iter().cycle() {
        sum += fr;
        if seen.contains(&sum) {
            return sum;
        }
        seen.insert(sum);
    }

    panic!("no solution found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1(vec![1, 1, 1]), 3);
        assert_eq!(solve1(vec![1, 1, -2]), 0);
        assert_eq!(solve1(vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(vec![1, -1]), 0);
        assert_eq!(solve2(vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(solve2(vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(solve2(vec![7, 7, -2, -7, -4]), 14);
    }
}
