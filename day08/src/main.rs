use std::{collections::HashMap, fs::read_to_string};

struct Space {
    positions: Vec<Position>,
}

impl Space {
    fn new(input: &str) -> Self {
        let positions: Vec<Position> = input
            .lines()
            .map(|line| Position::new(line.trim()))
            .collect();
        Space { positions }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(position_str: &str) -> Self {
        let coords: Vec<i64> = position_str
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        Position {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn distance_squared(&self, other: &Position) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = HashMap::new();

        for i in 0..n {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }

        sizes.values().copied().collect()
    }
}

fn main() {
    let input = match read_to_string("day08/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Part 1: {}", part1(&mut Space::new(&input)));
    println!("Part 2: {}", part2(&mut Space::new(&input)));
}

fn part1(space: &mut Space) -> usize {
    let n = space.positions.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = space.positions[i].distance_squared(&space.positions[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_by_key(|&(dist, _, _)| dist);

    let num_pairs = if n == 20 { 10 } else { 1000 };
    let mut uf = UnionFind::new(n);
    for idx in 0..num_pairs {
        let (_, i, j) = edges[idx];
        uf.union(i, j);
    }

    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn part2(space: &mut Space) -> i64 {
    let n = space.positions.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = space.positions[i].distance_squared(&space.positions[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_by_key(|&(dist, _, _)| dist);

    let mut uf = UnionFind::new(n);
    let mut idx = 0;
    loop {
        let (_, i, j) = edges[idx];
        uf.union(i, j);
        if uf.size.iter().any(|&s| s == n) {
            return space.positions[i].x * space.positions[j].x;
        }
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut Space::new(EXAMPLE.trim())), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut Space::new(EXAMPLE.trim())), 25272);
    }
}
