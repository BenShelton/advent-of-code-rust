use std::str::Lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 13);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 43);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

#[derive(Clone)]
struct Grid {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_lines(lines: &mut Lines) -> Grid {
        let line1 = lines.next().unwrap();
        let width = line1.len();
        let mut height = 1;
        let mut cells = line1.chars().collect::<Vec<char>>();
        for line in lines {
            height += 1;
            cells.extend(line.chars());
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: char) {
        let index = y * self.width + x;
        if let Some(c) = self.cells.get_mut(index) {
            *c = cell;
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&char> {
        let index = y * self.width + x;
        self.cells.get(index)
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<char> {
        let mut neighbours = Vec::new();
        for ny in y.saturating_sub(1)..=usize::min(y + 1, self.height - 1) {
            for nx in x.saturating_sub(1)..=usize::min(x + 1, self.width - 1) {
                if nx == x && ny == y {
                    continue;
                }
                if let Some(&cell) = self.get_cell(nx, ny) {
                    neighbours.push(cell);
                }
            }
        }
        neighbours
    }
}

fn part_one(file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let grid = Grid::from_lines(&mut file.lines());
    let mut total: u32 = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get_cell(x, y);
            if !matches!(cell, Some('@')) {
                continue;
            }
            let neighbours = grid.get_neighbours(x, y);
            let paper_count = neighbours.into_iter().filter(|c| c == &'@').count();
            if paper_count < 4 {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn part_two(file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut grid = Grid::from_lines(&mut file.lines());
    let mut total: u32 = 0;

    let mut did_remove = true;
    while did_remove {
        did_remove = false;
        let mut next_grid = grid.clone();
        for y in 0..grid.height {
            for x in 0..grid.width {
                let cell = grid.get_cell(x, y);
                if !matches!(cell, Some('@')) {
                    continue;
                }
                let neighbours = grid.get_neighbours(x, y);
                let paper_count = neighbours.into_iter().filter(|c| c == &'@').count();
                if paper_count < 4 {
                    total += 1;
                    did_remove = true;
                    next_grid.set_cell(x, y, '.');
                }
            }
        }
        grid = next_grid;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_lines() {
        let data = include_str!("data-sample.txt").trim();
        let grid = Grid::from_lines(&mut data.lines());
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(
            grid.cells,
            vec![
                '.', '.', '@', '@', '.', '@', '@', '@', '@', '.', // Line 1
                '@', '@', '@', '.', '@', '.', '@', '.', '@', '@', // Line 2
                '@', '@', '@', '@', '@', '.', '@', '.', '@', '@', // Line 3
                '@', '.', '@', '@', '@', '@', '.', '.', '@', '.', // Line 4
                '@', '@', '.', '@', '@', '@', '@', '.', '@', '@', // Line 5
                '.', '@', '@', '@', '@', '@', '@', '@', '.', '@', // Line 6
                '.', '@', '.', '@', '.', '@', '.', '@', '@', '@', // Line 7
                '@', '.', '@', '@', '@', '.', '@', '@', '@', '@', // Line 8
                '.', '@', '@', '@', '@', '@', '@', '@', '@', '.', // Line 9
                '@', '.', '@', '.', '@', '@', '@', '.', '@', '.', // Line 10
            ]
        );
    }

    #[test]
    fn test_grid_get_neighbours() {
        let data = include_str!("data-sample.txt").trim();
        let grid = Grid::from_lines(&mut data.lines());
        assert_eq!(grid.get_neighbours(0, 0), vec!['.', '@', '@']);
        assert_eq!(
            grid.get_neighbours(1, 1),
            vec!['.', '.', '@', '@', '@', '@', '@', '@']
        );
        assert_eq!(
            grid.get_neighbours(2, 5),
            vec!['@', '.', '@', '@', '@', '@', '.', '@']
        );
        assert_eq!(grid.get_neighbours(9, 0), vec!['@', '@', '@']);
        assert_eq!(grid.get_neighbours(0, 9), vec!['.', '@', '.']);
        assert_eq!(grid.get_neighbours(9, 9), vec!['@', '.', '@']);
    }
}
