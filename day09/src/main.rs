use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("day09/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let coords = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse::<i64>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect::<Vec<(i64, i64)>>();

    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let area =
                ((coords[i].0 - coords[j].0).abs() + 1) * ((coords[i].1 - coords[j].1).abs() + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn part2(input: &str) -> i64 {
    let coords = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse::<i64>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect::<Vec<(i64, i64)>>();

    let mut max_area = 0;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let min_x = coords[i].0.min(coords[j].0);
            let max_x = coords[i].0.max(coords[j].0);
            let min_y = coords[i].1.min(coords[j].1);
            let max_y = coords[i].1.max(coords[j].1);

            let corners = [
                (min_x, min_y),
                (min_x, max_y),
                (max_x, min_y),
                (max_x, max_y),
            ];

            if !corners.iter().all(|&c| is_point_inside_polygon(c, &coords)) {
                continue;
            }

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let sample_step = 50.max(width.max(height) / 20).min(1000);
            let mut all_samples_inside = true;

            let mut x = min_x;
            while x <= max_x && all_samples_inside {
                if !is_point_inside_polygon((x, min_y), &coords)
                    || !is_point_inside_polygon((x, max_y), &coords)
                {
                    all_samples_inside = false;
                }
                x = (x + sample_step).min(max_x);
                if x == max_x {
                    break;
                }
            }

            if all_samples_inside {
                let mut y = min_y;
                while y <= max_y && all_samples_inside {
                    if !is_point_inside_polygon((min_x, y), &coords)
                        || !is_point_inside_polygon((max_x, y), &coords)
                    {
                        all_samples_inside = false;
                    }
                    y = (y + sample_step).min(max_y);
                    if y == max_y {
                        break;
                    }
                }
            }

            if !all_samples_inside {
                continue;
            }

            let mut has_interior_red = false;
            for k in 0..coords.len() {
                if k == i || k == j {
                    continue;
                }
                let (x, y) = coords[k];
                let on_boundary = (x == min_x || x == max_x) && (y >= min_y && y <= max_y)
                    || (y == min_y || y == max_y) && (x >= min_x && x <= max_x);

                if !on_boundary && x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                    has_interior_red = true;
                    break;
                }
            }

            if !has_interior_red {
                let area = ((coords[i].0 - coords[j].0).abs() + 1)
                    * ((coords[i].1 - coords[j].1).abs() + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn is_point_inside_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[j];

        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            let cross = (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
            if cross == 0 {
                return true;
            }
        }
    }

    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 24);
    }
}
