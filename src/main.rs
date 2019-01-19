extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate image;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn read_and_parse_input<T>(file: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::fs::read_to_string(format!("inputs/{}", file))
        .unwrap()
        .lines()
        .map(|v| v.parse::<T>().unwrap())
        .collect()
}

// Read input unmodified.
pub fn read_input(file: &str) -> String {
    std::fs::read_to_string(format!("inputs/{}", file))
        .unwrap()
        .to_string()
}

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("missing day argument")
        .parse::<u64>()
        .expect("day must be integer");

    match day {
        1 => {
            println!(
                "day1 part1: {}",
                day1::solve1(read_and_parse_input("day1.txt"))
            );
            println!(
                "day1 part2: {}",
                day1::solve2(read_and_parse_input("day1.txt"))
            );
        }
        2 => {
            println!(
                "day2 part1: {}",
                day2::solve1(read_and_parse_input("day2.txt"))
            );
            println!(
                "day2 part2: {}",
                day2::solve2(read_and_parse_input("day2.txt"))
            );
        }
        3 => {
            println!(
                "day3 part1: {}",
                day3::solve1(read_and_parse_input("day3.txt"))
            );
            println!(
                "day3 part2: {}",
                day3::solve2(read_and_parse_input("day3.txt"))
            );
        }
        4 => {
            println!(
                "day4 part1: {}",
                day4::solve1(read_and_parse_input("day4.txt"))
            );
            println!(
                "day4 part2: {}",
                day4::solve2(read_and_parse_input("day4.txt"))
            );
        }
        5 => {
            println!("day5 part1: {}", day5::solve1(read_input("day5.txt")));
            println!("day5 part2: {}", day5::solve2(read_input("day5.txt")));
        }
        6 => {
            println!(
                "day6 part1: {}",
                day6::solve1(read_and_parse_input("day6.txt"))
            );
            println!(
                "day6 part2: {}",
                day6::solve2(read_and_parse_input("day6.txt"))
            );
        }
        7 => {
            println!(
                "day7 part1: {}",
                day7::solve1(read_and_parse_input("day7.txt"))
            );
            println!(
                "day7 part2: {}",
                day7::solve2(read_and_parse_input("day7.txt"), 5, 60)
            );
        }
        8 => {
            println!(
                "day8 part1: {}",
                day8::solve1(read_and_parse_input("day8.txt"))
            );
            println!(
                "day8 part2: {}",
                day8::solve2(read_and_parse_input("day8.txt"))
            );
        }
        9 => {
            println!("day9 part1: {}", day9::solve1(479, 71035));
            println!("day9 part2: {}", day9::solve1(479, 71035 * 100));
        }
        10 => {
            day10::solve1(read_and_parse_input("day10.txt"), 150_000, 58);
            println!("day10 part1: see image file.");
        }
        11 => {
            println!("day11 part1: {:?}", day11::solve1(9810));
            println!("day11 part2: {:?}", day11::solve2(9810));
        }
        12 => {
            println!("day12 part1: {}", day12::solve1(read_input("day12.txt")));
            println!("day12 part2: {}", day12::solve2(read_input("day12.txt")));
        }
        13 => {
            println!("day13 part1: {:?}", day13::solve1(read_input("day13.txt")));
            println!("day13 part2: {:?}", day13::solve2(read_input("day13.txt")));
        }
        14 => {
            println!("day14 part1: {}", day14::solve1(765071));
            println!("day14 part2: {}", day14::solve2("765071"));
        }
        15 => {
            println!("day15 part1: {}", day15::solve1(read_input("day15.txt")));
            println!("day15 part2: {}", day15::solve2(read_input("day15.txt")));
        }
        16 => {
            println!("day16 part1: {}", day16::solve1(read_input("day16.txt")));
            println!("day16 part2: {}", day16::solve2(read_input("day16.txt")));
        }
        17 => {
            let (p1, p2) = day17::solve1(read_and_parse_input("day17.txt"));
            println!("day17 part1: {} part2: {}", p1, p2);
        }
        18 => {
            println!("day18 part1: {}", day18::solve1(read_input("day18.txt")));
            println!("day18 part2: {}", day18::solve2(read_input("day18.txt")));
        }
        19 => {
            println!("day19 part1: {}", day19::solve1(read_input("day19.txt")));
            println!("day19 part2: {}", day19::solve2(read_input("day19.txt")));
        }
        20 => {
            println!("day20 part1: {}", day20::solve1(read_input("day20.txt")));
            println!("day20 part2: {}", day20::solve2(read_input("day20.txt")));
        }
        21 => {
            println!("day21 part1: {}", day21::solve1(read_input("day21.txt")));
            println!("day21 part2: {}", day21::solve2(read_input("day21.txt")));
        }
        22 => {
            println!("day22 part1: {}", day22::solve1(7740, 12, 763));
            println!("day22 part2: {}", day22::solve2(7740, 12, 763));
        }
        23 => {
            println!(
                "day23 part1: {}",
                day23::solve1(read_and_parse_input("day23.txt"))
            );
            println!(
                "day23 part2: {}",
                day23::solve2(read_and_parse_input("day23.txt"))
            );
        }
        24 => {
            println!("day24 part1: {}", day24::solve1(read_input("day24.txt")));
            println!("day24 part2: {}", day24::solve2(read_input("day24.txt")));
        }
        25 => {
            println!(
                "day25 part1: {}",
                day25::solve1(read_and_parse_input("day25.txt"))
            );
        }
        _ => panic!("invalid day"),
    }
}
