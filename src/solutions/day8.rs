use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

#[derive(Debug, Clone)]
struct Edge {
    i: usize,
    j: usize,
    distance: f32,
}

#[derive(Debug, Clone)]
struct Node {
    i: usize,
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

pub fn solution(coordinates: &[(u64, u64, u64)], conn_num: usize) -> anyhow::Result<u32> {
    // Possible paths with distances
    let mut paths = BinaryHeap::new();

    for (i, (x1, y1, z1)) in coordinates.iter().enumerate() {
        for (j, (x2, y2, z2)) in coordinates.iter().take(i).enumerate() {
            let distance = ((x1.abs_diff(*x2).pow(2)
                + y1.abs_diff(*y2).pow(2)
                + z1.abs_diff(*z2).pow(2)) as f32)
                .sqrt();

            let edge = Edge { i, j, distance };

            paths.push(ShortestPath(edge.clone()));
        }
    }

    let mut adjacency_list = Vec::with_capacity(coordinates.len());

    for i in 0..coordinates.len() {
        adjacency_list.push(Node {
            i,
            edges: Vec::new(),
        })
    }

    // Find 10 shortests paths
    let mut i = 0;
    while let Some(path) = paths.pop()
        && i < conn_num
    {
        i += 1;
        let ShortestPath(edge) = path;

        adjacency_list[edge.j].edges.push(edge.flip());
        adjacency_list[edge.i].edges.push(edge);
    }

    let mut discovered = Vec::new();
    let mut visited = HashSet::new();

    let mut graphs = BinaryHeap::new();

    // DFS
    for node in adjacency_list.iter() {
        if !visited.contains(&node.i) {
            discovered.push(node.i);
            let mut graph_len = 0u32;

            while let Some(next_node) = discovered.pop() {
                let next_node = &adjacency_list[next_node];
                next_node.edges.iter().for_each(|edge| {
                    if !visited.contains(&edge.j) && !discovered.contains(&edge.j) {
                        discovered.push(edge.j);
                    }
                });

                visited.insert(next_node.i);
                graph_len += 1;
            }
            graphs.push(graph_len);
        }
    }

    let mut product = 1u32;

    // Calculate product of 3 greatest graph lengths
    let mut i = 0;
    while let Some(graph_len) = graphs.pop()
        && i < 3
    {
        i += 1;

        product *= graph_len;
    }

    Ok(product)
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

    let result = solution(&coordinates, 1000)?;

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

        let result = solution(&testdata, 10).unwrap();
        assert_eq!(result, 40);
    }
}
