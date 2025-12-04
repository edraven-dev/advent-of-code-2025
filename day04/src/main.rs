use std::fs::read_to_string;

struct Grid {
    chars: Vec<Vec<char>>,
}

impl Grid {
    const PAPER: char = '@';
    const BLANK: char = '.';
    const ADJACENT_PAPERS_CONSTRAINT: u64 = 4;

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

    fn is_paper(&self, row: usize, col: usize) -> bool {
        match self.get_char(row, col) {
            Some(ch) if ch == Self::PAPER => true,
            _ => false,
        }
    }

    fn get_char(&self, row: usize, col: usize) -> Option<char> {
        if row < self.chars.len() && col < self.chars[row].len() {
            Some(self.chars[row][col])
        } else {
            None
        }
    }

    fn get_adjacent_papers_count(&self, row: usize, col: usize) -> u64 {
        let mut count = 0;

        for r in row.saturating_sub(1)..=(row + 1).min(self.get_row_size() - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.get_col_size() - 1) {
                if r == row && c == col {
                    continue;
                }
                if let Some(ch) = self.get_char(r, c) {
                    if ch == Self::PAPER {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn part1(grid: &Grid) -> u64 {
    let mut total_count = 0;

    for i in 0..grid.get_row_size() {
        for j in 0..grid.get_col_size() {
            if !grid.is_paper(i, j) {
                continue;
            }

            if grid.get_adjacent_papers_count(i, j) < Grid::ADJACENT_PAPERS_CONSTRAINT {
                total_count += 1;
            }
        }
    }

    total_count
}

fn part2(grid: &mut Grid) -> u64 {
    let mut total_count = 0;

    loop {
        let mut any_changes = false;
        for i in 0..grid.get_row_size() {
            for j in 0..grid.get_col_size() {
                if !grid.is_paper(i, j) {
                    continue;
                }

                if grid.get_adjacent_papers_count(i, j) < Grid::ADJACENT_PAPERS_CONSTRAINT {
                    total_count += 1;
                    any_changes = true;
                    grid.chars[i][j] = Grid::BLANK;
                }
            }
        }
        if !any_changes {
            break;
        }
    }

    total_count
}

fn main() {
    let input = match read_to_string("day04/input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let mut grid = Grid::new(&input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&Grid::new(EXAMPLE.trim())), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut Grid::new(EXAMPLE.trim())), 43);
    }
}
