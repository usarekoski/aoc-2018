fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let a = (rack_id * y + serial) * rack_id;
    let third_digit = (a / 10 / 10) % 10;
    third_digit - 5
}

type Grid = Vec<Vec<i64>>;

fn create_grid(serial: i64) -> Grid {
    let mut grid: Grid = vec![vec![0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = power_level(x as i64 + 1, y as i64 + 1, serial)
        }
    }

    grid
}

// Returns (x, y, power)
fn max_total_power(grid: &Grid, size: i64) -> (i64, i64, i64) {
    let end = 300 - size;

    let ((max_x, max_y), power) = (1..=end)
        .map(|y| (1..=end).map(move |x| (x, y)))
        .flatten()
        .map(|(x, y)| {
            let total_power: i64 = (0..size)
                .map(|x_offset| (0..size).map(move |y_offset| (x + x_offset, y + y_offset)))
                .flatten()
                .map(|(x1, y1)| grid[y1 as usize - 1][x1 as usize - 1])
                .sum();
            ((x, y), total_power)
        })
        .max_by_key(|((_, _), total_power)| *total_power)
        .unwrap();

    (max_x, max_y, power)
}

pub fn solve1(serial: i64) -> (i64, i64) {
    let grid = create_grid(serial);
    let (x, y, _) = max_total_power(&grid, 3);
    (x, y)
}

pub fn solve2(serial: i64) -> (i64, i64, i64) {
    let grid = create_grid(serial);
    let (size, (x, y, _)) = (1..300)
        .map(|size| (size, max_total_power(&grid, size)))
        .max_by_key(|(_, (_, _, power))| *power)
        .unwrap();

    (x, y, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(18), (33, 45));
        assert_eq!(solve1(42), (21, 61));
    }

    #[test]
    fn test2() {
        // assert_eq!(solve2(18), (90, 269, 16));
    }
}
