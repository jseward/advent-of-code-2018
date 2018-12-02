use std::collections::HashMap;

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

fn get_id_value(id: &String) -> u32 {
    let mut value: u32 = 0;
    for c in id.chars() {
        value += c as u32
    }
    value
}

fn find_id_diff(
    id_values: &HashMap<u32, Vec<String>>,
    id: &String,
    id_value: u32,
) -> Option<String> {
    for other_id_value in (id_value - 128)..(id_value + 128):
    Some("foo".to_string())
}

#[aoc(day2, part2)]
pub fn part2(input: &[String]) -> String {
    let mut id_values: HashMap<u32, Vec<String>> = HashMap::new();
    for id in input {
        let id_value = get_id_value(id);
        match find_id_diff(&id_values, &id, id_value) {
            Some(id_diff) => return id_diff,
            _ => {
                let mut values_vec = id_values.entry(id_value).or_insert(Vec::new());
                values_vec.push(id.to_string())
            }
        };
    }
    unreachable!()
}
