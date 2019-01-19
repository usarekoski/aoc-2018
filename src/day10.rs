use std::str::FromStr;

use regex::Regex;

pub struct Point {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

impl FromStr for Point {
    type Err = Box<::std::error::Error>;

    // input: position=< 9,  1> velocity=< 0,  2>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^position=<\s?(-?\d+), \s?(-?\d+)> velocity=<\s?(-?\d+), \s?(-?\d+)>$"
            )
            .unwrap();
        }
        let cap = RE.captures(s).unwrap();

        Ok(Point {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
            v_x: cap[3].parse()?,
            v_y: cap[4].parse()?,
        })
    }
}

pub struct Sky {
    points: Vec<Point>,
    size: usize,
    seconds: u64,
}

impl Sky {
    fn variance(&self) -> u64 {
        let mut x_count = vec![0; self.size];
        let mut y_count = vec![0; self.size];
        let offset: i64 = (self.size / 2) as i64;
        for p in &self.points {
            x_count[(p.x + offset) as usize] = 1;
            y_count[(p.y + offset) as usize] = 1;
        }

        x_count.iter().sum::<u64>() + y_count.iter().sum::<u64>()
    }

    fn forward(&mut self) {
        for p in &mut self.points {
            p.x += p.v_x;
            p.y += p.v_y;
        }
        self.seconds += 1;
    }

    fn backward(&mut self) {
        for p in &mut self.points {
            p.x -= p.v_x;
            p.y -= p.v_y;
        }
        self.seconds -= 1;
    }

    fn print(&self, name: &str) {
        let max_x = self.points.iter().max_by_key(|p| p.x).unwrap().x;
        let min_x = self.points.iter().min_by_key(|p| p.x).unwrap().x;
        let max_y = self.points.iter().max_by_key(|p| p.y).unwrap().y;
        let min_y = self.points.iter().min_by_key(|p| p.y).unwrap().y;
        let offset = min_x.min(0).abs().max(min_y.min(0).abs());
        let size = (max_x + offset).max(max_y + offset) as usize + 2;

        let mut sky = vec![vec![0u8; size]; size];
        for p in &self.points {
            if let Some(s1) = sky.get_mut((p.y + offset) as usize) {
                if let Some(s2) = s1.get_mut((p.x + offset) as usize) {
                    *s2 = 255u8;
                }
            }
        }
        let b = sky.iter().flatten().map(|s| *s).collect::<Vec<u8>>();

        image::save_buffer(
            &std::path::Path::new(&format!("outputs/{}.png", name)),
            &b,
            size as u32,
            size as u32,
            image::Gray(8),
        )
        .expect("writing image file failed");
    }
}

pub fn solve1(points: Vec<Point>, size: usize, variance: u64) {
    let mut sky = Sky {
        points,
        size,
        seconds: 0,
    };
    let mut last = sky.variance();
    let mut s_last = last;
    let mut cur = last;

    while cur != variance {
        // Use this to obtain variance value:
        // if cur != last {
        //     println!("variance: {}", cur);
        //     sky.print(&cur.to_string());
        // }
        sky.forward();
        s_last = last;
        last = cur;
        cur = sky.variance();
    }

    println!("variance: {} {} {}", cur, last, s_last);
    sky.print("day10_1");
    println!("seconds_1: {}", sky.seconds);
    sky.backward();
    sky.print("day10_2");
    println!("seconds_1: {}", sky.seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = [
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ]
        .iter()
        .map(|&s| String::from(s).parse::<Point>().unwrap())
        .collect();

        solve1(input, 50, 16);
        assert!(true);
    }
}
