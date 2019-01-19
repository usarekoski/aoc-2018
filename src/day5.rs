pub fn are_opposite((c1, c2): (char, char)) -> bool {
    if c1.is_lowercase() && c2.is_uppercase() {
        c2.to_ascii_lowercase() == c1
    } else if c1.is_uppercase() && c2.is_lowercase() {
        c1.to_ascii_lowercase() == c2
    } else {
        false
    }
}

pub fn solve1(mut polymers: String) -> u64 {
    polymers = polymers.trim().to_string();
    loop {
        let pos = polymers
            .chars()
            .zip(polymers.chars().skip(1))
            .position(are_opposite);

        if let Some(idx) = pos {
            polymers.remove(idx);
            polymers.remove(idx);
        } else {
            break;
        }
    }

    polymers.len() as u64
}

fn remove_units(polymers: &str, unit: char) -> String {
    polymers
        .chars()
        .filter(|&c| c != unit && c.to_ascii_lowercase() != unit)
        .collect()
}

// Solution: 6694
pub fn solve2(mut polymers: String) -> u64 {
    polymers = polymers.trim().to_string();
    ('a' as u32..='z' as u32)
        .map(|v| {
            let unit = std::char::from_u32(v).unwrap();
            solve1(remove_units(&polymers, unit))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "dabAcCaCBAcCcaDA".to_string();
        assert_eq!(solve1(input), 10);
    }

    #[test]
    fn test2() {
        let input = "dabAcCaCBAcCcaDA".to_string();
        assert_eq!(solve2(input), 4);
    }
}
