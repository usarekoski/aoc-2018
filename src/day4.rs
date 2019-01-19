use regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryInfo {
    GuardStarts(u64),
    FallsAsleep,
    WakesUp,
}

impl FromStr for EntryInfo {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
        }
        match s {
            "falls asleep" => Ok(EntryInfo::FallsAsleep),
            "wakes up" => Ok(EntryInfo::WakesUp),
            _ => {
                let cap = RE.captures(s).unwrap();
                Ok(EntryInfo::GuardStarts(cap[1].parse()?))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDate {
    year: u64,
    month: u64,
    day: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    date: SimpleDate,
    hours: u64,
    minutes: u64,
    content: EntryInfo,
}

impl FromStr for Entry {
    type Err = Box<::std::error::Error>;

    // [1518-06-08 00:36] wakes up
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (.+)$").unwrap();
        }

        let cap = RE.captures(s).unwrap();

        Ok(Entry {
            date: SimpleDate {
                year: cap[1].parse()?,
                month: cap[2].parse()?,
                day: cap[3].parse()?,
            },
            hours: cap[4].parse()?,
            minutes: cap[5].parse()?,
            content: cap[6].parse()?,
        })
    }
}

type AsleepMinutes = [u64; 60];

fn minutes_asleep(mut records: Vec<Entry>) -> HashMap<u64, AsleepMinutes> {
    // Sorting relies on struct field ordering.
    records.sort();

    let mut minutes_asleep: HashMap<u64, AsleepMinutes> = HashMap::new();
    let mut it = records.iter();
    let mut guard = None;

    while let Some(entry) = it.next() {
        match entry.content {
            EntryInfo::GuardStarts(id) => {
                guard = Some(id);
                if !minutes_asleep.contains_key(&id) {
                    minutes_asleep.insert(id, [0; 60]);
                }
            }
            EntryInfo::FallsAsleep => {
                let asleep = minutes_asleep.get_mut(&guard.unwrap()).unwrap();
                assert!(entry.hours == 0);
                for minute in (entry.minutes as usize)..60 {
                    asleep[minute] += 1;
                }
            }
            EntryInfo::WakesUp => {
                let asleep = minutes_asleep.get_mut(&guard.unwrap()).unwrap();
                assert!(entry.hours == 0);
                for minute in (entry.minutes as usize)..60 {
                    asleep[minute] -= 1;
                }
            }
        }
    }

    minutes_asleep
}

pub fn solve1(records: Vec<Entry>) -> u64 {
    let minutes_asleep = minutes_asleep(records);

    let (max_id, minutes) = minutes_asleep
        .iter()
        .max_by_key(|(_, v)| -> u64 { v.iter().sum() })
        .unwrap();

    let max_asleep_minutes = minutes.iter().max().unwrap();
    let max_asleep_minute = minutes
        .iter()
        .position(|v| v == max_asleep_minutes)
        .unwrap();

    println!("guard {}, minutes {}", max_id, max_asleep_minutes);
    println!("max_asleep_minute {}", max_asleep_minute);

    *max_id * (max_asleep_minute as u64)
}

pub fn solve2(records: Vec<Entry>) -> u64 {
    let minutes_asleep = minutes_asleep(records);

    let (guard_id, _) = minutes_asleep
        .iter()
        .max_by_key(|(_, v)| v.iter().max())
        .unwrap();

    let (minute, _) = minutes_asleep
        .get(guard_id)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|(_, &m)| m)
        .unwrap();

    *guard_id * minute as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let records = [
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .iter()
        .map(|&s| String::from(s).parse::<Entry>().unwrap())
        .collect();

        assert_eq!(solve1(records), 240);
    }

    #[test]
    fn test2() {
        let records = [
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .iter()
        .map(|&s| String::from(s).parse::<Entry>().unwrap())
        .collect();

        assert_eq!(solve2(records), 4455);
    }
}
