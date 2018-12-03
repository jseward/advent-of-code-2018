pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    use regex::Regex;
    // #1 @ 393,863: 11x29
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| Claim {
            id: cap[1].parse::<u32>().unwrap(),
            left: cap[2].parse::<u32>().unwrap(),
            top: cap[3].parse::<u32>().unwrap(),
            width: cap[4].parse::<u32>().unwrap(),
            height: cap[5].parse::<u32>().unwrap(),
        }).collect()
}

use std::collections::HashMap;
fn make_fabric_map(claims: &[Claim]) -> HashMap<(u32, u32), u32> {
    let mut fabric_map: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in claims.iter() {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                let count = fabric_map.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }
    fabric_map
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> u32 {
    make_fabric_map(input)
        .iter()
        .map(|(_, count)| match *count > 1 {
            true => 1,
            false => 0,
        }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> u32 {
    let fabric_map = make_fabric_map(input);
    for claim in input.iter() {
        let mut is_unique_claim = true;
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                match fabric_map[&(x, y)] {
                    1 => {}
                    _ => is_unique_claim = false,
                }
            }
        }
        match is_unique_claim {
            true => return claim.id,
            false => {}
        }
    }
    unreachable!()
}
