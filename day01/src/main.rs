use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("day01/input.txt") {
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
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();

    let starting_position = 50;
    let mut current_position = starting_position;
    let mut zero_occurrances = 0;

    for line in lines {
        let (dir, distance_str) = line.trim().split_at(1);
        let distance = distance_str.parse::<i64>().unwrap();
        match dir {
            "L" => current_position = (current_position - distance).rem_euclid(100),
            "R" => current_position = (current_position + distance).rem_euclid(100),
            _ => panic!("Invalid direction"),
        }

        if current_position == 0 {
            zero_occurrances += 1;
        }
    }

    zero_occurrances
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();

    let starting_position = 50;
    let mut current_position = starting_position;
    let mut zero_occurrances = 0;

    for line in lines {
        let (dir, distance_str) = line.trim().split_at(1);
        let distance = distance_str.parse::<i64>().unwrap();
        match dir {
            "L" => {
                let crossings = if current_position > 0 && distance >= current_position {
                    (distance - current_position) / 100 + 1
                } else if current_position == 0 {
                    distance / 100
                } else {
                    0
                };
                zero_occurrances += crossings;
                current_position = (current_position - distance).rem_euclid(100);
            }
            "R" => {
                let new_position = current_position + distance;
                zero_occurrances += new_position.div_euclid(100);
                current_position = new_position.rem_euclid(100);
            }
            _ => panic!("Invalid direction"),
        }
    }

    zero_occurrances
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.trim()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.trim()), 6);
    }
}
