use crate::util;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = util::read_input(2)?;

    println!(
        "Day 2, Part 1: {}\nDay 2, Part 2: {}",
        part_1(input.clone()),
        part_2(input)
    );

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn part_1(input: String) -> i32 {
    input.lines().map(parse_part1_line).sum()
}

fn parse_part1_line(line: &str) -> i32 {
    let split: Vec<_> = line.split(' ').collect();
    let opponent_choice = match split[0] {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid input"),
    };
    let my_choice = match split[1] {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Invalid input"),
    };

    score_hand(my_choice, judge_match(my_choice, opponent_choice))
}

fn score_hand(shape_played: Shape, outcome: Outcome) -> i32 {
    let shape_score = match shape_played {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let outcome_score = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    };

    shape_score + outcome_score
}

fn judge_match(your_shape: Shape, other: Shape) -> Outcome {
    match (your_shape, other) {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => Outcome::Win,

        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,

        _ => Outcome::Loss,
    }
}

fn part_2(input: String) -> i32 {
    input.lines().map(parse_part2_line).sum()
}

fn parse_part2_line(line: &str) -> i32 {
    let split: Vec<_> = line.split(' ').collect();
    let opponent_choice = match split[0] {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid input"),
    };
    let required_outcome = match split[1] {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid input"),
    };

    let my_shape = match (opponent_choice, required_outcome) {
        (_, Outcome::Draw) => opponent_choice,
        (Shape::Paper, Outcome::Loss) | (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Rock, Outcome::Loss) | (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Loss) | (Shape::Rock, Outcome::Win) => Shape::Paper,
    };

    score_hand(my_shape, required_outcome)
}

#[cfg(test)]
mod test {
    use crate::util::read_input;

    use super::*;
    const TEST_INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn test_test_input() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 15);
    }

    #[test]
    fn test_correct_answers() {
        let real_input = read_input(2).unwrap();
        assert_eq!(part_1(real_input.clone()), 13809);
        assert_eq!(part_2(real_input), 12316);
    }
}
