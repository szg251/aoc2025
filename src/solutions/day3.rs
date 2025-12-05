use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(banks: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u64> {
    const BATTERIES: usize = 12;

    let mut sum = 0u64;

    for bank in banks {
        let bank = bank?;

        let mut in_jolts = bank.bytes().map(|byte| byte - 48);

        let mut out_jolts = [0u8; BATTERIES];

        for out_jolt in out_jolts.iter_mut() {
            *out_jolt = in_jolts
                .next()
                .ok_or(anyhow!("there must be at least {} batteries", BATTERIES))?;
        }

        for digit in in_jolts {
            add_digit(&mut out_jolts, digit);
        }
        let max = String::from_utf8(
            out_jolts
                .into_iter()
                .map(|byte| byte + 48)
                .collect::<Vec<_>>(),
        )
        .context("couldn't convert back to string")?
        .parse::<u64>()
        .context("cannot parse largest joltage")?;

        sum += max;
    }
    Ok(sum)
}

fn add_digit(out_jolts: &mut [u8], digit: u8) {
    if out_jolts[1] > out_jolts[0] {
        // move everything to the left
        for j in 0..(out_jolts.len() - 1) {
            out_jolts[j] = out_jolts[j + 1];
        }
        out_jolts[out_jolts.len() - 1] = digit;
    } else {
        // do the same as above but with the next subslice
        let (_, rest) = out_jolts.split_at_mut(1);
        if rest.len() >= 2 {
            add_digit(rest, digit);
        } else if digit > rest[0] {
            rest[0] = digit;
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day3").context("couldn't open day 3 input file")?;
    let buf = BufReader::new(f);

    let banks = buf.lines().map(|line| line.context("couldn't read line"));

    let joltage = solution(banks)?;

    eprintln!("Day 3: {joltage}");

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

        assert_eq!(joltage, 3121910778619);
    }
}
