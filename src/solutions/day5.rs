use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(ranges: &[(u64, u64)]) -> anyhow::Result<u64> {
    let mut sum = 0u64;
    let mut normalized: Vec<(u64, u64)> = Vec::new();

    let mut index = 0usize;

    for insert in ranges {
        let to_merge = normalized
            .iter()
            .map(|(start, end)| (*start, *end))
            .enumerate()
            .skip_while(|(i, norm_range)| {
                index = *i;
                insert.0 > norm_range.1
            })
            .take_while(|(_, norm_range)| insert.1 >= norm_range.0)
            .collect::<Vec<(usize, (u64, u64))>>();

        let new_start = if let Some(old) = to_merge.first().map(|(_, r)| r)
            && old.0 < insert.0
        {
            old.0
        } else {
            insert.0
        };

        let new_end = if let Some(old) = to_merge.last().map(|(_, r)| r)
            && old.1 > insert.1
        {
            old.1
        } else {
            insert.1
        };

        if !to_merge.is_empty() {
            let start = to_merge.first().unwrap().0;
            let end = to_merge.last().unwrap().0;
            normalized.drain(start..=end);
        }

        // If the item is the last, then the skip_while step might not have given us the correct
        // index
        if index + 1 == normalized.len() && normalized[normalized.len() - 1].0 < new_start {
            normalized.push((new_start, new_end));
        } else {
            normalized.insert(index, (new_start, new_end));
        }
        dbg!(&normalized);
    }

    for range in normalized {
        sum += range.1 - range.0 + 1;
    }
    Ok(sum)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day5").context("couldn't open day 5 input file")?;
    let lines = BufReader::new(f).lines();

    let ranges = lines
        .take_while(|line| match line {
            Ok(line) => !line.is_empty(),
            _ => false,
        })
        .map(|line| {
            let line = line.context("couldn't read line")?;
            let (start, end) = line
                .split_once('-')
                .ok_or(anyhow!("string must be a range with a - delimiter"))?;
            let start = start.parse::<u64>().context("couldn't parse range start")?;
            let end = end.parse::<u64>().context("couldn't parse range end")?;

            Ok((start, end))
        })
        .collect::<anyhow::Result<Vec<(u64, u64)>>>()?;

    let sum = solution(&ranges)?;
    eprintln!("Day 5: {sum}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let ranges = [(11, 13), (3, 5), (10, 14), (16, 20), (12, 18)];

        let result = solution(&ranges).unwrap();

        assert_eq!(result, 14);
    }
}
