use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(matrix: &[Vec<String>]) -> anyhow::Result<u64> {
    let mut total = 0u64;
    let last_row = matrix.last().ok_or(anyhow!(
        "there must be at least one input row with operators"
    ))?;
    for (i, op) in last_row.iter().enumerate() {
        let op_identity = match &op[..] {
            "*" => Ok::<u64, anyhow::Error>(1),
            "+" => Ok(0),
            _ => Err(anyhow!("unknown operator {op}"))?,
        };
        let op = |x, y| match &op[..] {
            "*" => Ok::<u64, anyhow::Error>(x * y),
            "+" => Ok(x + y),
            _ => Err(anyhow!("unknown operator {op}"))?,
        };

        total += matrix
            .iter()
            .take(matrix.len() - 1)
            .try_fold(op_identity?, |result, xs| {
                let x = xs[i].parse::<u64>().context("couldn't parse number")?;
                op(x, result)
            })?;
    }
    Ok(total)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day6").context("couldn't open day 6 file")?;
    let buf = BufReader::new(f);

    let matrix = buf
        .lines()
        .map(|line| {
            let line = line.context("couldn't read line")?;
            let words = line
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            Ok(words)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let total = solution(&matrix)?;

    eprintln!("Day 6: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            ["123", "328", "51", "64"],
            ["45", "64", "387", "23"],
            ["6", "98", "215", "314"],
            ["*", "+", "*", "+"],
        ];

        let sum = solution(
            &testdata
                .into_iter()
                .map(|xs| xs.into_iter().map(String::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
        .unwrap();

        assert_eq!(sum, 4277556);
    }
}
