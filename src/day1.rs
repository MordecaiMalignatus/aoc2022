use crate::util;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = util::read_input(1)?;

    println!(
        "Day 1, Part 1: {}\nDay 1, Part 2: {}",
        part_1(input.clone()),
        part_2(input)
    );

    Ok(())
}

pub fn part_1(input: String) -> i32 {
    let mut res = input
        .split("\n\n")
        .map(|t| {
            t.trim()
                .split('\n')
                .map(|el| el.parse::<i32>().unwrap())
                .reduce(|acc, el| acc + el)
                .unwrap()
        })
        .collect::<Vec<i32>>();

    res.sort();
    res[res.len() - 1]
}

pub fn part_2(input: String) -> i32 {
    let mut res = input
        .split("\n\n")
        .map(|t| {
            t.trim()
                .split('\n')
                .map(|el| el.parse::<i32>().unwrap())
                .reduce(|acc, el| acc + el)
                .unwrap()
        })
        .collect::<Vec<i32>>();

    res.sort();
    res.iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
        .into();
        let res = part_1(input);
        assert_eq!(res, 24_000);
    }
}
