use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

pub fn solution(lines: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u32> {
    let mut beams = HashSet::new();
    let mut splits = 0u32;

    for line in lines {
        let line = line?;

        // let line_len = line.len();

        for (i, char) in line.char_indices() {
            match char {
                'S' => {
                    beams.insert(i);
                }
                '^' => {
                    if beams.contains(&i) {
                        dbg!(&i);
                        if i > 0 {
                            beams.insert(i - 1)
                        } else {
                            false
                        };
                        if i < line.len() - 1 {
                            beams.insert(i + 1)
                        } else {
                            false
                        };

                        beams.remove(&i);

                        splits += 1;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(splits)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day7").context("couldn't open day 7 input")?;
    let buf = BufReader::new(f);

    let lines = buf.lines().map(|line| line.context("couldn't read line"));

    let res = solution(lines)?;

    eprintln!("Day 7: {res}");

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let splits = solution(testdata.into_iter().map(String::from).map(Ok)).unwrap();
        assert_eq!(splits, 21);
    }
}

