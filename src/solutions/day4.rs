use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

pub fn solution(in_rows: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u32> {
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for in_row in in_rows {
        let in_row = in_row?;
        let mut row = Vec::new();
        for col in in_row.chars() {
            match col {
                '@' => row.push(1),
                _ => row.push(0),
            }
        }

        matrix.push(row);
    }

    let mut sum = 0u32;

    loop {
        let rolls = compute_once(&mut matrix)?;
        if rolls == 0 {
            break;
        }
        sum += rolls;
    }

    Ok(sum)
}

pub fn compute_once(matrix: &mut [Vec<u8>]) -> anyhow::Result<u32> {
    let mut precomputed: Vec<Vec<u8>> = Vec::new();
    let mut rolls = 0u32;

    for row in matrix.iter() {
        let mut precomputed_row = Vec::new();
        precomputed_row.push(row[0] + row[1]);

        row.windows(3)
            .for_each(|items| precomputed_row.push(items.iter().sum()));

        precomputed_row.push(row[row.len() - 2] + row[row.len() - 1]);

        precomputed.push(precomputed_row);
    }

    let row_len = matrix.len();

    for (y, row) in matrix.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if *cell == 0 {
                continue;
            }
            let up_row_sum = if y == 0 { 0 } else { precomputed[y - 1][x] };
            let middle_row_sum = precomputed[y][x];
            let down_row_sum = if y == row_len - 1 {
                0
            } else {
                precomputed[y + 1][x]
            };

            if up_row_sum + middle_row_sum + down_row_sum <= 4 {
                rolls += 1;
                *cell = 0;
            }
        }
    }

    Ok(rolls)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day4").context("couldn't open day4 input")?;
    let buf = BufReader::new(f);

    let sum = solution(buf.lines().map(|line| line.context("couldn't read line")))?;

    eprintln!("Day 4: {sum}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        let sum = solution(testdata.into_iter().map(String::from).map(Ok)).unwrap();

        assert_eq!(sum, 43);
    }
}
