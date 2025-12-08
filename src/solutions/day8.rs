use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, anyhow};

#[derive(Debug, Clone)]
struct Edge {
    i: usize,
    j: usize,
    distance: f32,
}

#[derive(Debug, Clone)]
struct Node {
    i: usize,
    x: u64,
    y: u64,
    z: u64,
    edges: Vec<Edge>,
}

impl Edge {
    fn flip(&self) -> Self {
        Edge {
            i: self.j,
            j: self.i,
            distance: self.distance,
        }
    }
}

struct ShortestPath(Edge);

impl Eq for ShortestPath {}

impl PartialEq for ShortestPath {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance == other.0.distance
    }
}

impl PartialOrd for ShortestPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortestPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.distance.partial_cmp(&self.0.distance).unwrap()
    }
}

pub fn solution(coordinates: &[(u64, u64, u64)]) -> anyhow::Result<u64> {
    // Possible paths with distances
    let mut paths = BinaryHeap::new();

    let mut adjacency_list = coordinates
        .iter()
        .enumerate()
        .map(|(i, (x, y, z))| Node {
            i,
            x: *x,
            y: *y,
            z: *z,
            edges: Vec::new(),
        })
        .collect::<Vec<_>>();

    for n1 in adjacency_list.iter() {
        for n2 in adjacency_list.iter().take(n1.i) {
            let distance = ((n1.x.abs_diff(n2.x).pow(2)
                + n1.y.abs_diff(n2.y).pow(2)
                + n1.z.abs_diff(n2.z).pow(2)) as f32)
                .sqrt();

            let edge = Edge {
                i: n1.i,
                j: n2.i,
                distance,
            };

            paths.push(ShortestPath(edge.clone()));
        }
    }

    let mut discovered = Vec::new();
    let mut visited = HashSet::new();

    if let Some(ShortestPath(edge)) = paths.pop() {
        adjacency_list[edge.i].edges.push(edge.clone());
        adjacency_list[edge.j].edges.push(edge.flip());

        discovered.push(edge.i);
    }

    while let Some(path) = paths.pop() {
        let ShortestPath(edge) = path;

        adjacency_list[edge.i].edges.push(edge.clone());
        adjacency_list[edge.j].edges.push(edge.flip());

        // DFS

        if !visited.contains(&edge.i) && visited.contains(&edge.j) {
            discovered.push(edge.i);
        }

        if !visited.contains(&edge.j) && visited.contains(&edge.i) {
            discovered.push(edge.j);
        }

        while let Some(next_node) = discovered.pop() {
            let next_node = &adjacency_list[next_node];
            next_node.edges.iter().for_each(|edge| {
                if !visited.contains(&edge.j) && !discovered.contains(&edge.j) {
                    discovered.push(edge.j);
                }
            });

            visited.insert(next_node.i);
        }
        if visited.len() == coordinates.len() {
            return Ok(adjacency_list[edge.i].x * adjacency_list[edge.j].x);
        }
    }

    Err(anyhow!("couldn't connect all the nodes"))
}

pub fn run() -> anyhow::Result<()> {
    let f = File::open("inputs/day8").context("couldn't open day 8 input")?;
    let buf = BufReader::new(f);

    let mut coordinates = Vec::new();

    for line in buf.lines() {
        let line = line.context("couldn't read line")?;
        let nums = line
            .splitn(3, ',')
            .map(|str| str.parse::<u64>().context("couldn't parse input number"))
            .collect::<anyhow::Result<Vec<_>>>()?;

        coordinates.push((nums[0], nums[1], nums[2]));
    }

    let result = solution(&coordinates)?;

    eprintln!("Day 8: {result}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test() {
        let testdata = [
            (162, 817, 812),
            (57, 618, 57),
            (906, 360, 560),
            (592, 479, 940),
            (352, 342, 300),
            (466, 668, 158),
            (542, 29, 236),
            (431, 825, 988),
            (739, 650, 466),
            (52, 470, 668),
            (216, 146, 977),
            (819, 987, 18),
            (117, 168, 530),
            (805, 96, 715),
            (346, 949, 466),
            (970, 615, 88),
            (941, 993, 340),
            (862, 61, 35),
            (984, 92, 344),
            (425, 690, 689),
        ];

        let result = solution(&testdata).unwrap();
        assert_eq!(result, 25272);
    }
}
