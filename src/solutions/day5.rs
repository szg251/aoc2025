use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(ranges: &[(u64, u64)], ids: &[u64]) -> anyhow::Result<u64> {
    let mut sum = 0u64;
    for id in ids {
        if ranges
            .iter()
            .any(|(start, end)| (start..=end).contains(&id))
        {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day5").context("couldn't open day 5 input file")?;
    let mut lines = BufReader::new(f).lines();

    let ranges = lines
        .by_ref()
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

    // lines.next();

    let ids = lines
        .map(|line| {
            line.context("couldn't read line")?
                .parse::<u64>()
                .context("could'nt parse id")
        })
        .collect::<anyhow::Result<Vec<u64>>>()?;

    let sum = solution(&ranges, &ids)?;
    eprintln!("Day 5: {sum}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];

        let ids = [1, 5, 8, 11, 17, 32];

        let result = solution(&ranges, &ids).unwrap();

        assert_eq!(result, 3);
    }
}
