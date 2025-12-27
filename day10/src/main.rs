use std::collections::HashMap;
use std::fs::read_to_string;

use rand::{Rng, rng};

#[derive(Debug)]
struct Machine {
    pattern: String,
    buttons: Vec<Button>,
    joltages: Vec<u64>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let (pattern, buttons, joltages) =
            input
                .split_whitespace()
                .fold(("", Vec::new(), Vec::new()), |mut acc, token| {
                    if token.starts_with('[') && token.ends_with(']') {
                        acc.0 = &token[1..token.len() - 1];
                    } else if token.starts_with('(') && token.ends_with(')') {
                        let positions: Vec<usize> = token[1..token.len() - 1]
                            .split(',')
                            .map(|s| s.trim().parse::<usize>().unwrap())
                            .collect();
                        acc.1.push(Button { positions });
                    } else if token.starts_with('{') && token.ends_with('}') {
                        // Parse joltages
                        let js: Vec<u64> = token[1..token.len() - 1]
                            .split(',')
                            .map(|s| s.trim().parse::<u64>().unwrap())
                            .collect();
                        acc.2 = js;
                    }
                    acc
                });

        Machine {
            pattern: pattern.to_string(),
            buttons: buttons,
            joltages: joltages,
        }
    }
}

#[derive(Debug)]
struct Button {
    positions: Vec<usize>,
}

fn main() {
    let input = match read_to_string("day10/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let machines = input
        .lines()
        .map(|line| Machine::new(line))
        .collect::<Vec<Machine>>();

    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines));
}

fn part1(machines: &Vec<Machine>) -> usize {
    const SAMPLES: usize = 10000;
    const MAX_STEPS: usize = 10;

    let mut steps_per_machine = Vec::new();

    for machine in machines {
        let desired_state = machine
            .pattern
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        let mut steps = MAX_STEPS + 1;

        for _ in 0..SAMPLES {
            let mut current_state = vec![false; machine.pattern.len()];
            for i in 0..MAX_STEPS {
                let button_idx = rng().random_range(0..machine.buttons.len());
                let button = &machine.buttons[button_idx];
                for &pos in &button.positions {
                    if pos < current_state.len() {
                        current_state[pos] = !current_state[pos];
                    }
                }
                if current_state == desired_state {
                    steps = steps.min(i + 1);
                    break;
                }
            }
        }

        if steps == MAX_STEPS + 1 {
            panic!(
                "Machine {:?} could not complete any sample in {} steps",
                machine, MAX_STEPS
            );
        }
        steps_per_machine.push(steps);
    }

    // println!("Steps per machine: {:?}", steps_per_machine);
    steps_per_machine.iter().sum()
}

fn part2(machines: &Vec<Machine>) -> usize {
    let mut steps_per_machine = Vec::new();

    for machine in machines {
        let steps = joltage_cost(&machine.buttons, &machine.joltages).expect("No solution found");
        steps_per_machine.push(steps);
    }

    // println!("Steps per machine: {:?}", steps_per_machine);
    steps_per_machine.iter().sum()
}

fn joltage_cost(buttons: &[Button], joltage: &[u64]) -> Option<usize> {
    let n = joltage.len();

    let mut press_patterns: HashMap<Vec<u64>, Vec<Vec<usize>>> = HashMap::new();

    let n_buttons = buttons.len();
    for mask in 0..(1 << n_buttons) {
        let mut combo = Vec::new();
        for i in 0..n_buttons {
            if mask & (1 << i) != 0 {
                combo.push(i);
            }
        }

        let pattern = pattern(&press(&combo, buttons, n));
        press_patterns
            .entry(pattern)
            .or_insert_with(Vec::new)
            .push(combo);
    }

    let mut cache: HashMap<Vec<u64>, usize> = HashMap::new();

    fn cost_recursive(
        jolts: &[u64],
        press_patterns: &HashMap<Vec<u64>, Vec<Vec<usize>>>,
        buttons: &[Button],
        target_sum: usize,
        cache: &mut HashMap<Vec<u64>, usize>,
    ) -> usize {
        if jolts.iter().all(|&j| j == 0) {
            return 0;
        }

        let jolts_vec = jolts.to_vec();
        if let Some(&cached) = cache.get(&jolts_vec) {
            return cached;
        }

        let pat = pattern(jolts);

        if !press_patterns.contains_key(&pat) {
            return target_sum;
        }

        let mut min_cost = target_sum;

        for btn_combo in &press_patterns[&pat] {
            let pressed = press(btn_combo, buttons, jolts.len());

            let can_subtract = jolts.iter().zip(pressed.iter()).all(|(&a, &b)| a >= b);
            if !can_subtract {
                continue;
            }

            let halved = sub_halve(jolts, &pressed);

            let sub_cost = cost_recursive(&halved, press_patterns, buttons, target_sum, cache);
            let total = btn_combo.len() + 2 * sub_cost;
            min_cost = min_cost.min(total);
        }

        cache.insert(jolts_vec, min_cost);
        min_cost
    }

    let target_sum: usize = joltage.iter().map(|&x| x as usize).sum();
    let result = cost_recursive(joltage, &press_patterns, buttons, target_sum, &mut cache);

    if result >= target_sum {
        None
    } else {
        Some(result)
    }
}

fn press(btn_indices: &[usize], buttons: &[Button], n_positions: usize) -> Vec<u64> {
    let mut result = vec![0u64; n_positions];
    for &btn_idx in btn_indices {
        for &pos in &buttons[btn_idx].positions {
            if pos < n_positions {
                result[pos] += 1;
            }
        }
    }
    result
}

fn pattern(jolts: &[u64]) -> Vec<u64> {
    jolts.iter().map(|&j| j % 2).collect()
}

fn sub_halve(j_a: &[u64], j_b: &[u64]) -> Vec<u64> {
    j_a.iter()
        .zip(j_b.iter())
        .map(|(&a, &b)| (a - b) / 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        let machines = EXAMPLE
            .lines()
            .map(|line| Machine::new(line))
            .collect::<Vec<Machine>>();

        assert_eq!(part1(&machines), 7);
    }

    #[test]
    fn test_part2() {
        let machines = EXAMPLE
            .lines()
            .map(|line| Machine::new(line))
            .collect::<Vec<Machine>>();

        assert_eq!(part2(&machines), 33);
    }
}
