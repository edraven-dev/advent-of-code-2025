use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Shape {
    _grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Shape {
    fn new(lines: Vec<&str>) -> Self {
        let grid: Vec<Vec<bool>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        Shape {
            _grid: grid,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

impl Region {
    fn new(width: usize, height: usize, shape_counts: Vec<usize>) -> Self {
        Region {
            width,
            height,
            shape_counts,
        }
    }
}

#[derive(Debug)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut shapes = Vec::new();
        let mut regions = Vec::new();

        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.ends_with(':') {
                let mut shape_lines = Vec::new();
                i += 1;

                while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') {
                    shape_lines.push(lines[i]);
                    i += 1;
                }

                shapes.push(Shape::new(shape_lines));
            } else if line.contains('x') && line.contains(':') {
                // Parse region definition (e.g., "4x4: 0 0 0 0 2 0")
                let parts: Vec<&str> = line.split(':').collect();
                let dimensions: Vec<usize> = parts[0]
                    .trim()
                    .split('x')
                    .map(|s| s.parse().unwrap())
                    .collect();

                let shape_counts: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                regions.push(Region::new(dimensions[0], dimensions[1], shape_counts));
                i += 1;
            } else {
                i += 1;
            }
        }

        Input { shapes, regions }
    }
}

fn main() {
    let input_str = match read_to_string("day12/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let input = Input::parse(&input_str);

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    let mut total = 0;

    for region in &input.regions {
        let shapes_size = region
            .shape_counts
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, &count)| {
                acc + input.shapes[idx].width * input.shapes[idx].height * count
            });

        if shapes_size <= region.width * region.height {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn test_part1() {
        let input = Input::parse(EXAMPLE.trim());
        assert_eq!(part1(&input), 1);
    }
}
