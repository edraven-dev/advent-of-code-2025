use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("day06/input.txt") {
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
    let mut signs = Vec::new();
    let mut nums = Vec::new();
    input.lines().for_each(|line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(i, num)| {
                match num.trim().parse::<u64>() {
                    Ok(v) => {
                        if nums.len() <= i {
                            nums.push(Vec::new());
                        }
                        nums[i].push(v);
                    }
                    Err(_e) => signs.push(num.trim()),
                };
            });
    });

    nums.iter().enumerate().fold(0u64, |acc, (i, col)| {
        let sign = signs.get(i).unwrap();
        let col_result = match *sign {
            "+" => col.iter().sum::<u64>(),
            "*" => col.iter().product::<u64>(),
            _ => acc,
        };
        acc + col_result
    })
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let nrows = lines.len();
    let ncols = lines[0].len();
    let mut results = Vec::new();

    let mut current_numbers = Vec::new();
    let mut current_sign = None;

    let mut col = 0;
    while col < ncols {
        let mut is_empty = true;
        for i in 0..nrows {
            let c = lines[i].chars().nth(col).unwrap_or(' ');
            if c != ' ' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            if !current_numbers.is_empty() && current_sign.is_some() {
                let res = match current_sign.unwrap() {
                    '+' => current_numbers.iter().sum::<u64>(),
                    '*' => current_numbers.iter().product::<u64>(),
                    _ => 0,
                };
                results.push(res);
            }
            current_numbers.clear();
            current_sign = None;
            col += 1;
            continue;
        }

        let mut col_digits = String::new();
        let mut sign = None;
        for row in 0..nrows {
            let c = lines[row].chars().nth(col).unwrap_or(' ');
            if c == '+' || c == '*' {
                sign = Some(c);
            } else if c.is_ascii_digit() {
                col_digits.push(c);
            } else if !col_digits.is_empty() {
                // End of number in this column
                if let Ok(num) = col_digits.parse::<u64>() {
                    current_numbers.push(num);
                }
                col_digits.clear();
            }
        }
        if !col_digits.is_empty() {
            if let Ok(num) = col_digits.parse::<u64>() {
                current_numbers.push(num);
            }
        }
        if sign.is_some() {
            current_sign = sign;
        }
        col += 1;
    }
    if !current_numbers.is_empty() && current_sign.is_some() {
        let res = match current_sign.unwrap() {
            '+' => current_numbers.iter().sum::<u64>(),
            '*' => current_numbers.iter().product::<u64>(),
            _ => 0,
        };
        results.push(res);
    }
    results.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 3263827);
    }
}
