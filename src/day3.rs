use std::collections::{hash_map::RandomState, HashSet};

use anyhow::Result;

use crate::util;

pub fn run() -> Result<()> {
    let input = util::read_input(3)?;

    println!(
        "Day 3, Part 1: {}\nDay 3, Part 2: {}",
        part_1(input.clone()),
        part_2(input)
    );

    Ok(())
}

fn part_1(input: String) -> i32 {
    input
        .lines()
        .map(backpack_overlap)
        .map(|overlap: Vec<char>| overlap.iter().map(item_to_priority).sum::<i32>())
        .sum()
}

fn backpack_overlap(line: &str) -> Vec<char> {
    let (backpack1, backpack2) = split_backpack(line);
    let b1: HashSet<char, RandomState> = HashSet::from_iter(backpack1.chars());
    let b2: HashSet<char, RandomState> = HashSet::from_iter(backpack2.chars());
    b1.intersection(&b2).map(|c| c.to_owned()).collect()
}

fn split_backpack(backpack: &str) -> (&str, &str) {
    let len = backpack.len();
    assert!(len % 2 == 0);
    let splitpoint = len / 2;

    (&backpack[..splitpoint], &backpack[splitpoint..])
}

fn item_to_priority(item: &char) -> i32 {
    (match item {
        'a'..='z' => *item as u8 - 96,
        'A'..='Z' => *item as u8 - 64 + 26,
        _ => panic!("Invalid item code"),
    }) as i32
}

fn part_2(input: String) -> i32 {
    input
        .trim()
        .lines()
        .array_chunks::<3>()
        .map(find_badge_for_group)
        .map(|c| item_to_priority(&c))
        .sum()
}

fn find_badge_for_group(group: [&str; 3]) -> char {
    let intersection = intersect(group);
    assert!(intersection.len() == 1);
    intersection[0]
}

fn intersect(input: [&str; 3]) -> Vec<char> {
    let [b1, b2, b3] = input.map(|line| HashSet::from_iter(line.chars()));
    b1.intersection(&b2)
        .map(|c| c.to_owned())
        .collect::<HashSet<char, RandomState>>()
        .intersection(&b3)
        .map(|c| c.to_owned())
        .collect::<Vec<char>>()
}

#[cfg(test)]
mod test {
    use crate::util::read_input;

    use super::*;

    const TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_test_input() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 157);
        assert_eq!(part_2(TEST_INPUT.to_string()), 70);
    }

    #[test]
    fn test_correct_answers() {
        let input = read_input(3).unwrap();

        assert_eq!(part_1(input.clone()), 8515);
        assert_eq!(part_2(input), 2434);
    }
}
