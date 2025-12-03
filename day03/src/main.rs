use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("day03/input.txt") {
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
    find_largest_number_of_n_digits(input, 2)
}

fn part2(input: &str) -> u64 {
    find_largest_number_of_n_digits(input, 12)
}

fn find_largest_number_of_n_digits(input: &str, n: usize) -> u64 {
    let banks = input.lines().collect::<Vec<_>>();

    let mut total_count = 0;
    for bank in banks {
        let digits = bank
            .chars()
            .into_iter()
            .map(|str_digit| str_digit.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut selected_digits: Vec<u32> = Vec::new();
        let mut current_start = 0;

        while selected_digits.len() < n && current_start < digits.len() {
            let remaining_digits_needed = n - selected_digits.len();
            let end_index = digits.len() - remaining_digits_needed + 1;

            if current_start >= end_index {
                break;
            }

            let mut max_value = 0;
            let mut max_index = current_start;
            for i in current_start..end_index {
                if digits[i] > max_value {
                    max_value = digits[i];
                    max_index = i;
                }
            }

            selected_digits.push(max_value);
            current_start = max_index + 1;
        }

        let mut result = 0u64;
        for i in 0..selected_digits.len() {
            result = result * 10 + selected_digits[i] as u64;
        }
        total_count += result;
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 3121910778619);
    }
}
