use std::fs::read_to_string;

#[derive(Clone)]
struct Grid {
    chars: Vec<Vec<char>>,
}

impl Grid {
    const STARTING_POINT: char = 'S';
    const BEAM: char = '|';
    const BLANK: char = '.';
    const SPLITTER: char = '^';

    fn new(input: &str) -> Self {
        let cells = input
            .lines()
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { chars: cells }
    }

    fn get_col_size(&self) -> usize {
        if self.chars.is_empty() {
            0
        } else {
            self.chars[0].len()
        }
    }

    fn get_row_size(&self) -> usize {
        self.chars.len()
    }

    fn get_char_above(&self, row: usize, col: usize) -> Option<char> {
        if row == 0 {
            None
        } else {
            self.get_char(row - 1, col)
        }
    }

    fn get_char(&self, row: usize, col: usize) -> Option<char> {
        if row < self.chars.len() && col < self.chars[row].len() {
            Some(self.chars[row][col])
        } else {
            None
        }
    }

    fn set_char(&mut self, row: usize, col: usize, ch: char) {
        if row < self.chars.len() && col < self.chars[row].len() {
            self.chars[row][col] = ch;
        }
    }
}

fn main() {
    let input = match read_to_string("day07/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Part 1: {}", part1(&mut Grid::new(&input)));
    println!("Part 2: {}", part2(&mut Grid::new(&input)));
}

fn part1(grid: &mut Grid) -> u64 {
    let mut total_count = 0;

    for i in 1..grid.get_row_size() {
        for j in 0..grid.get_col_size() {
            let current_char = grid.get_char(i, j).unwrap();
            let above_char = grid.get_char_above(i, j).unwrap();

            if current_char == Grid::BLANK
                && (above_char == Grid::STARTING_POINT || above_char == Grid::BEAM)
            {
                grid.set_char(i, j, Grid::BEAM);
            }

            if current_char == Grid::SPLITTER && above_char == Grid::BEAM {
                total_count += 1;
            }

            if j == 0 {
                continue;
            }

            if let Some(prev_char) = grid.get_char(i, j - 1) {
                if current_char == Grid::BLANK && prev_char == Grid::SPLITTER {
                    grid.set_char(i, j, Grid::BEAM);
                }
                if current_char == Grid::SPLITTER && prev_char == Grid::BLANK {
                    grid.set_char(i, j - 1, Grid::BEAM);
                }
            }
        }
    }

    total_count
}

fn part2(grid: &mut Grid) -> u64 {
    let mut start_col = 0;
    for j in 0..grid.get_col_size() {
        if grid.get_char(0, j) == Some(Grid::STARTING_POINT) {
            start_col = j;
            break;
        }
    }

    let mut cache: std::collections::HashMap<(usize, usize), u64> =
        std::collections::HashMap::new();

    count_paths(grid, 1, start_col, &mut cache)
}

fn count_paths(
    grid: &Grid,
    row: usize,
    col: usize,
    cache: &mut std::collections::HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(row, col)) {
        return cached;
    }

    let current_col = col;
    for r in row..grid.get_row_size() {
        let ch = grid.get_char(r, current_col).unwrap_or(Grid::BLANK);
        if ch == Grid::SPLITTER {
            let mut paths = 0;

            if current_col > 0 {
                let left_paths = count_paths_from_split(grid, r, current_col - 1, cache);
                paths += left_paths;
            }

            if current_col + 1 < grid.get_col_size() {
                let right_paths = count_paths_from_split(grid, r, current_col + 1, cache);
                paths += right_paths;
            }

            cache.insert((row, col), paths);
            return paths;
        }
    }

    cache.insert((row, col), 1);
    1
}

fn count_paths_from_split(
    grid: &Grid,
    splitter_row: usize,
    beam_col: usize,
    cache: &mut std::collections::HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(splitter_row, beam_col)) {
        return cached;
    }

    let mut paths = 0;
    let current_col = beam_col;

    for r in (splitter_row + 1)..grid.get_row_size() {
        let ch = grid.get_char(r, current_col).unwrap_or(Grid::BLANK);
        if ch == Grid::SPLITTER {
            if current_col > 0 {
                paths += count_paths_from_split(grid, r, current_col - 1, cache);
            }
            if current_col + 1 < grid.get_col_size() {
                paths += count_paths_from_split(grid, r, current_col + 1, cache);
            }
            cache.insert((splitter_row, beam_col), paths);
            return paths;
        }
    }

    cache.insert((splitter_row, beam_col), 1);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut Grid::new(EXAMPLE.trim())), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut Grid::new(EXAMPLE.trim())), 40);
    }
}
