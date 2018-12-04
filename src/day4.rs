use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    hour: u32,
    minute: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    date: Date,
    time: Time,
    action: Action,
}

#[derive(Debug)]
pub struct Sleep {
    guard_id: u32,
    date: Date,
    begin: Time,
    end: Time,
}

impl Sleep {
    fn minutes(&self) -> u32 {
        self.end.minute - self.begin.minute
    }
}

pub fn parse_action(s: &str) -> Action {
    match s {
        "falls asleep" => Action::FallAsleep,
        "wakes up" => Action::WakeUp,
        s => {
            // Guard #2789 begins shift
            let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
            let cap = re.captures(s).unwrap();
            Action::BeginShift(cap[1].parse::<u32>().unwrap())
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Sleep> {
    // [1518-07-04 00:01] falls asleep
    // [1518-06-18 00:02] Guard #2789 begins shift
    let re = Regex::new(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.+)").unwrap();
    let mut records = re
        .captures_iter(input)
        .map(|cap| Record {
            date: Date {
                year: cap[1].parse::<u32>().unwrap(),
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[3].parse::<u32>().unwrap(),
            },
            time: Time {
                hour: cap[4].parse::<u32>().unwrap(),
                minute: cap[5].parse::<u32>().unwrap(),
            },
            action: parse_action(&cap[6]),
        }).collect::<Vec<Record>>();
    records.sort_unstable();

    let mut sleeps: Vec<Sleep> = Vec::new();
    let mut guard_id: Option<u32> = None;
    let mut begin: Option<Record> = None;
    for record in records {
        match record.action {
            Action::BeginShift(begin_guard_id) => guard_id = Some(begin_guard_id),
            Action::FallAsleep => begin = Some(record.clone()),
            Action::WakeUp => {
                assert_eq!(begin.as_ref().unwrap().date, record.date);
                sleeps.push(Sleep {
                    guard_id: guard_id.unwrap(),
                    date: record.date,
                    begin: begin.as_ref().unwrap().time.clone(),
                    end: record.time,
                });
            }
        }
    }
    sleeps
}

#[aoc(day4, part1)]
pub fn part1(sleeps: &[Sleep]) -> u32 {
    let mut guard_to_total_sleep_duration: HashMap<u32, u32> = HashMap::new();
    for sleep in sleeps {
        let e = guard_to_total_sleep_duration
            .entry(sleep.guard_id)
            .or_insert(0);
        *e += sleep.minutes()
    }

    let sleepiest_guard_id = *guard_to_total_sleep_duration
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;

    let mut minute_counts: HashMap<u32, u32> = HashMap::new();
    for sleep in sleeps.iter().filter(|s| s.guard_id == sleepiest_guard_id) {
        for m in sleep.begin.minute..sleep.end.minute {
            let e = minute_counts.entry(m).or_insert(0);
            *e += 1
        }
    }

    let sleepiest_minute = minute_counts
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;

    sleepiest_guard_id * sleepiest_minute
}

#[aoc(day4, part2)]
pub fn part2(sleeps: &[Sleep]) -> u32 {
    let mut minute_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for sleep in sleeps {
        for m in sleep.begin.minute..sleep.end.minute {
            let e = minute_map.entry(m).or_insert(Vec::new());
            (*e).push(sleep.guard_id)
        }
    }

    let mut best_guard_id: Option<u32> = None;
    let mut best_minute: Option<u32> = None;
    let mut best_count: Option<u32> = None;
    for (minute, guard_ids) in minute_map {
        let mut unique_guard_ids: HashSet<u32> = HashSet::new();
        for guard_id in guard_ids.iter() {
            unique_guard_ids.insert(*guard_id);
        }
        for guard_id in unique_guard_ids {
            let count: u32 = (guard_ids.iter().filter(|x| **x == guard_id).count()) as u32;
            let is_better = match best_count {
                None => true,
                Some(v) => count > v,
            };
            if is_better {
                best_guard_id = Some(guard_id);
                best_minute = Some(minute);
                best_count = Some(count);
            }
        }
    }

    best_guard_id.unwrap() * best_minute.unwrap()
}
