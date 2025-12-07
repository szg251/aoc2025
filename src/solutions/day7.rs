use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

pub fn solution(lines: impl Iterator<Item = anyhow::Result<String>>) -> anyhow::Result<u64> {
    let mut beams = HashMap::new();

    for line in lines {
        let line = line?;

        for (i, char) in line.char_indices() {
            match char {
                'S' => {
                    beams.insert(i, 1u64);
                }
                '^' => {
                    if let Some(parent) = beams.get(&i).cloned() {
                        if i > 0 {
                            beams
                                .entry(i - 1)
                                .and_modify(|x| *x += parent)
                                .or_insert(parent);
                        };
                        if i < line.len() - 1 {
                            beams
                                .entry(i + 1)
                                .and_modify(|x| *x += parent)
                                .or_insert(parent);
                        };

                        beams.remove(&i);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(beams.values().sum())
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
        assert_eq!(splits, 40);
    }
}
