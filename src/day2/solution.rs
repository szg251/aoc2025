use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

pub fn solution(ranges: impl Iterator<Item = anyhow::Result<(u64, u64)>>) -> anyhow::Result<u64> {
    let mut sum = 0u64;
    for range in ranges {
        let (start, end) = range?;
        for num in start..=end {
            let num_str = num.to_string();
            let len = num_str.len();
            for i in (1..=(len / 2)).filter(|i| len % i == 0) {
                let mut chunks = num_str.as_bytes().chunks(i);
                let first = chunks
                    .next()
                    .ok_or(anyhow!("there must be at least one chunk in a string"))?;

                if chunks.all(|chunk| chunk == first) {
                    sum += num;
                    break;
                }
            }
        }
    }
    Ok(sum)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("src/day2/input").context("open day 2 input")?;
    let buf = BufReader::new(f);

    let input = buf.split(b',').map(|bytes| {
        let bytes = bytes?;
        let string = std::str::from_utf8(&bytes)
            .context("couldn't parse as ut8")?
            .trim_end();
        let mut slices = string.splitn(2, ['-']);
        let start: u64 = slices
            .next()
            .ok_or(anyhow!("couldn't find range start"))?
            .parse()
            .context("couldn't parse range start")?;

        let end: u64 = slices
            .next()
            .ok_or(anyhow!("couldn't find range end"))?
            .parse()
            .context("couldn't parse range end")?;
        Ok((start, end))
    });

    let sum = solution(input)?;

    eprintln!("{sum}");
    Ok(())
}
//

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];
        let sum = solution(testdata.into_iter().map(Ok)).unwrap();

        assert_eq!(sum, 4174379265);
    }
}
