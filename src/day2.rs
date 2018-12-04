#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

struct IdScore {
    has_two: bool,
    has_three: bool,
}

fn get_id_score(id: &str) -> IdScore {
    use std::collections::HashMap;

    let mut char_counts: HashMap<char, u32> = HashMap::new();
    for c in id.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut has_two = false;
    let mut has_three = false;
    for (_, count) in &char_counts {
        if *count == 2 {
            has_two = true;
        } else if *count == 3 {
            has_three = true;
        }
    }

    IdScore {
        has_two: has_two,
        has_three: has_three,
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> i32 {
    let mut has_two_count = 0;
    let mut has_three_count = 0;
    for id in input {
        let score = get_id_score(&id);
        if score.has_two {
            has_two_count += 1
        }
        if score.has_three {
            has_three_count += 1
        }
    }
    has_two_count * has_three_count
}

fn get_id_diff(id_1: &String, id_2: &String) -> Option<String> {
    let mut id_diff = String::new();
    let mut diff_count = 0;
    let iter = id_1.chars().zip(id_2.chars());
    for (c0, c1) in iter {
        match c0 == c1 {
            true => id_diff.push_str(&c0.to_string()),
            false => diff_count += 1,
        }
    }
    match diff_count {
        1 => Some(id_diff),
        _ => None,
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &[String]) -> String {
    let mut other_ids: Vec<String> = Vec::new();
    for id in input {
        for other_id in &other_ids {
            match get_id_diff(&id, &other_id) {
                Some(id_diff) => return id_diff,
                None => {}
            }
        }
        other_ids.push(id.to_string())
    }
    unreachable!()
}
