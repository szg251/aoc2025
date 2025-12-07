use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

#[derive(Debug, Clone, Copy)]
enum Op {
    Sum,
    Prod,
}

pub fn solution(mut lines: Vec<String>) -> anyhow::Result<u64> {
    let mut total = 0u64;

    let last_row = lines.split_off(lines.len() - 1).pop().ok_or(anyhow!(
        "there must be at least one input row with operators"
    ))?;

    let mut op = Op::Sum;
    let mut operands = Vec::new();

    let last_row = last_row.chars().enumerate();
    for (i, op_char) in last_row {
        match op_char {
            '*' => op = Op::Prod,
            '+' => op = Op::Sum,
            ' ' => {}
            _ => Err(anyhow!("unknown operator {op:?}"))?,
        };

        let column = lines
            .iter_mut()
            .try_fold(String::new(), |mut result, chars| {
                let next = chars
                    .as_bytes()
                    .get(i)
                    .context("there must be exactly equivalent length rows")?;
                result.push(*next as char);
                Ok::<String, anyhow::Error>(result)
            })?;

        let column = column.trim();

        if column.is_empty() {
            total += match op {
                Op::Prod => operands.iter().product::<u64>(),
                Op::Sum => operands.iter().sum(),
            };
            operands.clear();
        } else {
            operands.push(column.parse::<u64>().context("parsing operand")?);
        }
    }

    total += match op {
        Op::Prod => operands.iter().product::<u64>(),
        Op::Sum => operands.iter().sum(),
    };

    Ok(total)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day6").context("couldn't open day 6 file")?;
    let buf = BufReader::new(f);

    let lines = buf
        .lines()
        .map(|line| line.context("couldn't read line"))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let total = solution(lines)?;

    eprintln!("Day 6: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let sum = solution(testdata.into_iter().map(String::from).collect::<Vec<_>>()).unwrap();

        assert_eq!(sum, 3263827);
    }
}
