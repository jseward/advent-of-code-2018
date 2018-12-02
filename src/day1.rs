use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut i = 0;
    let mut x = 0;
    let mut s = HashSet::new();
    loop {
        if s.contains(&x) {
            return x;
        }
        s.insert(x);
        x = x + input[i];
        i = i + 1;
        if i >= input.len() {
            i = 0;
        }
    }
}
