use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Debug)]
struct Device {
    label: String,
    outputs: Vec<String>,
}

impl Device {
    fn new(input: &str) -> Self {
        let (label, outputs) = input
            .split_once(": ")
            .map(|(l, o)| {
                let outs: Vec<String> = o.split_whitespace().map(|s| s.to_string()).collect();
                (l, outs)
            })
            .unwrap();

        Device {
            label: label.to_string(),
            outputs: outputs,
        }
    }
}

fn main() {
    let input = match read_to_string("day11/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let devices = input
        .lines()
        .map(|line| Device::new(line))
        .collect::<Vec<Device>>();

    println!("Part 1: {}", part1(&devices));
    println!("Part 2: {}", part2(&devices));
}

fn part1(devices: &Vec<Device>) -> usize {
    let mut queue = VecDeque::new();

    let starting_node = devices.iter().find(|d| d.label == "you").unwrap();
    queue.push_back(starting_node);

    let mut total = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for label in &current.outputs {
            if label == "out" {
                total += 1;
                continue;
            }
            if let Some(next_device) = devices.iter().find(|d| &d.label == label) {
                queue.push_back(next_device);
            }
        }
    }

    total
}

fn part2(devices: &Vec<Device>) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for device in devices {
        graph.insert(
            &device.label,
            device.outputs.iter().map(|s| s.as_str()).collect(),
        );
    }

    type State<'a> = (&'a str, bool, bool);
    let mut dp: HashMap<State, Option<usize>> = HashMap::new();

    fn count_paths<'a>(
        node: &'a str,
        has_dac: bool,
        has_fft: bool,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        dp: &mut HashMap<State<'a>, Option<usize>>,
        in_stack: &mut HashSet<State<'a>>,
    ) -> usize {
        let new_has_dac = has_dac || node == "dac";
        let new_has_fft = has_fft || node == "fft";
        let state = (node, new_has_dac, new_has_fft);

        if in_stack.contains(&state) {
            return 0;
        }

        if let Some(&Some(result)) = dp.get(&state) {
            return result;
        }

        in_stack.insert(state);

        let outputs = match graph.get(node) {
            Some(o) => o,
            None => {
                in_stack.remove(&state);
                return 0;
            }
        };

        let mut total = 0;
        for &next in outputs {
            if next == "out" {
                if new_has_dac && new_has_fft {
                    total += 1;
                }
            } else {
                total += count_paths(next, new_has_dac, new_has_fft, graph, dp, in_stack);
            }
        }

        in_stack.remove(&state);
        dp.insert(state, Some(total));
        total
    }

    let mut in_stack = HashSet::new();
    count_paths("svr", false, false, &graph, &mut dp, &mut in_stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        let devices = example
            .lines()
            .map(|line| Device::new(line))
            .collect::<Vec<Device>>();

        assert_eq!(part1(&devices), 5);
    }

    #[test]
    fn test_part2() {
        let example = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        let devices = example
            .lines()
            .map(|line| Device::new(line))
            .collect::<Vec<Device>>();

        assert_eq!(part2(&devices), 2);
    }
}
