use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

type Point = (u64, u64);

type Line = (Point, Point);

/// Check for intersections, but allowing touching at the ends
fn intersect(l1: Line, l2: Line) -> bool {
    l1.0 != l2.0
        && l1.0 != l2.1
        && l1.1 != l2.0
        && l1.1 != l2.1
        && orientation(l1.0, l1.1, l2.0) != orientation(l1.0, l1.1, l2.1)
        && orientation(l2.0, l2.1, l1.0) != orientation(l2.0, l2.1, l1.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Clockwise,
    CounterClockwise,
    Collinear,
}

fn orientation(p1: Point, p2: Point, p3: Point) -> Orientation {
    let val = ((p2.1 as i128 - p1.1 as i128) * (p3.0 as i128 - p2.0 as i128))
        - ((p2.0 as i128 - p1.0 as i128) * (p3.1 as i128 - p2.1 as i128));

    if val == 0 {
        Orientation::Collinear
    } else if val > 0 {
        Orientation::Clockwise
    } else {
        Orientation::CounterClockwise
    }
}

pub fn solution(tiles: &[Point]) -> anyhow::Result<u64> {
    let mut largest = 0u64;

    for (i, t1) in tiles.iter().enumerate() {
        for t2 in tiles.iter().take(i) {
            let t3 = (t1.0, t2.1);
            let t4 = (t2.0, t1.1);

            let mut borders = tiles.iter().cloned().enumerate().peekable();
            let mut valid = true;

            while let Some((i1, b1)) = borders.next()
                && let Some((i2, b2)) = borders.peek()
            {
                if i1 != i && *i2 != i {
                    let border = (b1, *b2);
                    valid = valid && !intersect((*t1, *t2), border) && !intersect((t3, t4), border);
                    if !valid {
                        break;
                    }
                }
            }

            if valid {
                let area = (1 + t1.0.abs_diff(t2.0)) * (1 + (t1.1.abs_diff(t2.1)));
                if area > largest {
                    largest = area;
                }
            }
        }
    }

    Ok(largest)
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day9").context("couldn't open day 9 input")?;
    let buf = BufReader::new(f);

    let mut tiles = Vec::new();
    for line in buf.lines() {
        let line = line.context("couldn't read line")?;
        let (col, row) = line.split_once(',').ok_or(anyhow!(
            "there must be 2 numbers for each line separated by a colon "
        ))?;

        let col = col.parse::<u64>().context("couldn't parse column")?;
        let row = row.parse::<u64>().context("couldn't parse row")?;

        tiles.push((col, row));
    }

    let result = solution(&tiles)?;

    eprintln!("Day 9: {result}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Orientation, orientation, solution};

    #[test]
    fn orientation_test() {
        let x1 = 0;
        let y1 = 0;

        let x2 = 4;
        let y2 = 4;

        let x3 = 1;
        let y3 = 1;

        let res = orientation((x1, y1), (x2, y2), (x3, y3));

        assert_eq!(res, Orientation::Collinear);
    }
    #[test]
    fn test() {
        let testdata = [
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        let result = solution(&testdata).unwrap();
        assert_eq!(result, 24);
    }
}
