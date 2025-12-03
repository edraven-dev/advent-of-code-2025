use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("day02/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Part 2 (optimized): {}", part2_optimized(&input));
}

fn part1(input: &str) -> i64 {
    let ranges = input.split(",").collect::<Vec<_>>();

    let mut total_count = 0;
    for range in ranges {
        let (start_str, end_str) = range.trim().split_once("-").unwrap();
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str.parse::<i64>().unwrap();

        for num in start..=end {
            let num_str = num.to_string();
            let num_len = num_str.len();
            if num_len % 2 != 0 {
                continue;
            }
            let (left_part, right_part) = num_str.split_at(num_len / 2);
            if left_part.ends_with(right_part) {
                total_count += num;
            }
        }
    }

    total_count
}

fn part2(input: &str) -> i64 {
    let ranges = input.split(",").collect::<Vec<_>>();

    let mut total_count = 0;
    for range in ranges {
        let (start_str, end_str) = range.trim().split_once("-").unwrap();
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str.parse::<i64>().unwrap();

        for num in start..=end {
            let num_str = num.to_string();
            let num_len = num_str.len();
            for i in (1..=num_len / 2).rev() {
                if num_len % i == 0 {
                    let chunks = num_str
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(i)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect::<Vec<_>>();
                    if chunks.windows(2).all(|w| w[0] == w[1]) {
                        total_count += num;
                        break;
                    }
                }
            }
        }
    }

    total_count
}

fn part2_optimized(input: &str) -> i64 {
    use std::collections::HashSet;

    let ranges = input.split(",").collect::<Vec<_>>();

    let mut total_count = 0;
    for range in ranges {
        let (start_str, end_str) = range.trim().split_once("-").unwrap();
        let start = start_str.parse::<i64>().unwrap();
        let end = end_str.parse::<i64>().unwrap();

        let mut counted: HashSet<i64> = HashSet::new();

        let start_digits = if start == 0 {
            1
        } else {
            (start as f64).log10().floor() as usize + 1
        };
        let end_digits = (end as f64).log10().floor() as usize + 1;

        for total_len in start_digits..=end_digits {
            for chunk_size in 1..=total_len / 2 {
                if total_len % chunk_size != 0 {
                    continue;
                }
                let num_chunks = total_len / chunk_size;
                if num_chunks < 2 {
                    continue;
                }

                // Generate all base patterns of `chunk_size` digits
                // Base pattern ranges from 10^(chunk_size-1) to 10^chunk_size - 1
                // (or from 0 if chunk_size == 1, but we need leading zeros handled)
                let base_start = if chunk_size == 1 {
                    0
                } else {
                    10_i64.pow((chunk_size - 1) as u32)
                };
                let base_end = 10_i64.pow(chunk_size as u32);

                for base in base_start..base_end {
                    let pattern = format!("{:0>width$}", base, width = chunk_size);
                    let num_str = pattern.repeat(num_chunks);

                    // Skip if it has leading zeros (would be a different length number)
                    if num_str.starts_with('0') {
                        continue;
                    }

                    let num: i64 = num_str.parse().unwrap();

                    // Check if the number is in range and not already counted
                    if num >= start && num <= end && !counted.contains(&num) {
                        counted.insert(num);
                        total_count += num;
                    }
                }
            }
        }
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 4174379265);
    }

    #[test]
    fn test_part2_optimized() {
        assert_eq!(part2_optimized(EXAMPLE.trim()), 4174379265);
    }
}
