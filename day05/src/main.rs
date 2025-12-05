use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = match read_to_string("day05/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();

    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut ranges_processed = false;
    for line in lines {
        if ranges_processed == false && line != "" {
            line.trim()
                .split_once("-")
                .into_iter()
                .for_each(|(start, end)| {
                    let start = start.parse::<u64>().unwrap();
                    let end = end.parse::<u64>().unwrap();
                    ranges.push((start, end));
                });
        } else if line == "" {
            ranges_processed = true;
        } else {
            ids.push(line.trim().parse::<u64>().unwrap());
        }
    }

    let mut total_count = 0;
    for range in ranges {
        let (start, end) = range;
        let mut i = 0;
        while i < ids.len() {
            if ids[i] >= start && ids[i] <= end {
                total_count += 1;
                ids.remove(i);
            } else {
                i += 1;
            }
        }
    }

    total_count
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();

    let mut ranges = Vec::new();

    for line in lines {
        line.trim()
            .split_once("-")
            .into_iter()
            .for_each(|(start, end)| {
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                ranges.push((start, end));
            });
        if line == "" {
            break;
        }
    }

    let mut new_ranges = Vec::new();

    for range in ranges {
        let (mut start, mut end) = range;

        let mut overlaps_indices = HashSet::new();

        for (i, (ns, ne)) in new_ranges.iter().enumerate() {
            if (start >= *ns && start <= *ne)
                || (end >= *ns && end <= *ne)
                || (*ns >= start && *ns <= end)
                || (*ne >= start && *ne <= end)
            {
                overlaps_indices.insert(i);
            }
        }

        if overlaps_indices.is_empty() {
            new_ranges.push((start, end));
        } else {
            for &i in overlaps_indices.iter() {
                let (ns, ne) = new_ranges[i];
                if ns < start {
                    start = ns;
                }
                if ne > end {
                    end = ne;
                }
            }

            let mut overlaps_indices_vec: Vec<usize> = overlaps_indices.into_iter().collect();
            overlaps_indices_vec.sort_unstable_by(|a, b| b.cmp(a));
            for i in overlaps_indices_vec {
                new_ranges.remove(i);
            }

            new_ranges.push((start, end));
        }
    }

    new_ranges
        .iter()
        .fold(0, |acc, (start, end)| acc + (end - start + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 14);
    }
}
