use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(banks: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u64> {
    let mut sum = 0u64;
    for bank in banks {
        let bank = bank?;

        let mut bank_joltages = bank.bytes().map(|byte| byte - 48);

        let mut first = bank_joltages
            .next()
            .ok_or(anyhow!("there must be at least two batteries"))?;
        let mut second = bank_joltages
            .next()
            .ok_or(anyhow!("there must be at least two batteries"))?;

        for digit in bank_joltages {
            if second > first {
                first = second;
                second = digit;
            } else if digit > second {
                second = digit;
            }
        }
        let max = [first.to_string(), second.to_string()]
            .join("")
            .parse::<u64>()
            .context("cannot parse largest joltage")?;

        dbg!(max);
        sum += max;
    }
    Ok(sum)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("src/day3").context("couldn't open day 3 input file")?;
    let buf = BufReader::new(f);

    let banks = buf.lines().map(|line| line.context("couldn't read line"));

    let joltage = solution(banks)?;

    eprintln!("{joltage}");

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::solution;

        let testdata = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        let joltage = solution(testdata.into_iter().map(String::from).map(Ok)).unwrap();

        assert_eq!(joltage, 357);
    }
}
