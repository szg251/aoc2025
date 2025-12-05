use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use anyhow::anyhow;

pub fn solution(lines: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u32> {
    let mut dial = 50i16;
    let mut next = 0i16;
    let mut count = 0u32;

    for line in lines {
        let line = line?;
        let (direction, amount) = line.split_at(1);
        let amount = amount.parse::<i16>().context("parsing amount")?;

        match direction {
            "L" => {
                next = dial - amount;
            }
            "R" => {
                next = dial + amount;
            }
            _ => Err(anyhow!("invalid input: {}", direction))?,
        }

        count += (next / 100).unsigned_abs() as u32;
        if dial != 0 && next <= 0 {
            count += 1;
        }
        next = next.rem_euclid(100);
        dial = next;
    }

    Ok(count)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day1").context("open day 1 input")?;
    let buf = BufReader::new(f);

    let lines = buf.lines().map(|line| line.context("reading input line"));

    let count = solution(lines)?;

    eprintln!("Day 1: {count}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "R501",
        ];

        let count = solution(testdata.into_iter().map(String::from).map(Ok)).unwrap();
        assert_eq!(count, 11);
    }
}
