use rand::Rng;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Path,
}

pub struct CylinderMaze {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl CylinderMaze {
    pub fn new(rows: usize, cols: usize) -> Self {
        // Create grid with walls and paths: (2*rows + 1) x (2*cols + 1)
        // Odd positions are cells, even positions are walls
        let grid_rows = 2 * rows + 1;
        let grid_cols = 2 * cols + 1;
        CylinderMaze {
            grid: vec![vec![Cell::Wall; grid_cols]; grid_rows],
            rows,
            cols,
        }
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    fn cell_to_grid(&self, row: usize, col: usize) -> (usize, usize) {
        (2 * row + 1, 2 * col + 1)
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        // Up
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        // Down
        if row < self.rows - 1 {
            neighbors.push((row + 1, col));
        }
        // Left (wraps around cylinder)
        let left_col = if col == 0 { self.cols - 1 } else { col - 1 };
        neighbors.push((row, left_col));

        // Right (wraps around cylinder)
        let right_col = (col + 1) % self.cols;
        neighbors.push((row, right_col));

        neighbors
    }

    fn carve_passage(&mut self, from: (usize, usize), to: (usize, usize)) {
        let (from_r, from_c) = self.cell_to_grid(from.0, from.1);
        let (to_r, to_c) = self.cell_to_grid(to.0, to.1);

        // Mark both cells as paths
        self.grid[from_r][from_c] = Cell::Path;
        self.grid[to_r][to_c] = Cell::Path;

        // Mark the wall between them as path
        // Handle wrapping for horizontal movement
        if from.0 == to.0 {
            // Horizontal movement
            if (from.1 == 0 && to.1 == self.cols - 1) || (from.1 == self.cols - 1 && to.1 == 0) {
                // Wrapping around cylinder - connect through leftmost and rightmost walls
                // The leftmost wall (column 0) and rightmost wall (column grid_cols-1) are the same
                let grid_cols = self.grid[0].len();
                self.grid[from_r][0] = Cell::Path;
                self.grid[from_r][grid_cols - 1] = Cell::Path;
            } else {
                let wall_c = (from_c + to_c) / 2;
                self.grid[from_r][wall_c] = Cell::Path;
            }
        } else {
            // Vertical movement
            let wall_r = (from_r + to_r) / 2;
            self.grid[wall_r][from_c] = Cell::Path;
        }
    }

    pub fn generate_wilson(&mut self) -> ((usize, usize), (usize, usize)) {
        let mut rng = rand::thread_rng();
        let mut in_maze = HashSet::new();

        // Start with a random cell in the top row
        let start_row = 0;
        let start_col = rng.gen_range(0..self.cols);
        in_maze.insert((start_row, start_col));
        let (gr, gc) = self.cell_to_grid(start_row, start_col);
        self.grid[gr][gc] = Cell::Path;

        // Add all other cells
        for row in 0..self.rows {
            for col in 0..self.cols {
                if in_maze.contains(&(row, col)) {
                    continue;
                }

                // Perform loop-erased random walk
                let mut path = vec![(row, col)];
                let mut current = (row, col);

                while !in_maze.contains(&current) {
                    let neighbors = self.get_neighbors(current.0, current.1);
                    let next = neighbors[rng.gen_range(0..neighbors.len())];

                    // Check if we've visited this cell in current walk
                    if let Some(pos) = path.iter().position(|&p| p == next) {
                        // Loop detected - erase the loop
                        path.truncate(pos + 1);
                    } else {
                        path.push(next);
                    }

                    current = next;
                }

                // Add the path to the maze by carving passages
                for i in 0..path.len() {
                    let cell = path[i];
                    in_maze.insert(cell);

                    if i > 0 {
                        self.carve_passage(path[i - 1], cell);
                    }
                }
            }
        }

        // Pick a random cell in the bottom row as the end
        let end_row = self.rows - 1;
        let end_col = rng.gen_range(0..self.cols);

        ((start_row, start_col), (end_row, end_col))
    }

    pub fn display(&self, start: (usize, usize), end: (usize, usize)) {
        let (start_r, start_c) = self.cell_to_grid(start.0, start.1);
        let (end_r, end_c) = self.cell_to_grid(end.0, end.1);

        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if (r, c) == (start_r, start_c) {
                    print!("S");
                } else if (r, c) == (end_r, end_c) {
                    print!("E");
                } else {
                    match cell {
                        Cell::Wall => print!("█"),
                        Cell::Path => print!(" "),
                    }
                }
            }
            println!();
        }
    }

    pub fn can_solve(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_r, start_c) = self.cell_to_grid(start.0, start.1);
        let (end_r, end_c) = self.cell_to_grid(end.0, end.1);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start_r, start_c));
        visited.insert((start_r, start_c));

        let grid_rows = self.grid.len();
        let grid_cols = self.grid[0].len();

        while let Some((r, c)) = queue.pop_front() {
            if (r, c) == (end_r, end_c) {
                return true;
            }

            // Check all four directions
            let mut neighbors = Vec::new();

            // Up
            if r > 0 {
                neighbors.push((r - 1, c));
            }
            // Down
            if r + 1 < grid_rows {
                neighbors.push((r + 1, c));
            }
            // Left (with wrapping)
            let left_c = if c == 0 { grid_cols - 1 } else { c - 1 };
            neighbors.push((r, left_c));

            // Right (with wrapping)
            let right_c = (c + 1) % grid_cols;
            neighbors.push((r, right_c));

            for (nr, nc) in neighbors {
                if !visited.contains(&(nr, nc)) && self.grid[nr][nc] == Cell::Path {
                    visited.insert((nr, nc));
                    queue.push_back((nr, nc));
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_is_solvable() {
        // Generate multiple mazes and verify they're all solvable
        for _ in 0..10 {
            let mut maze = CylinderMaze::new(10, 10);
            let (start, end) = maze.generate_wilson();

            assert!(
                maze.can_solve(start, end),
                "Maze should be solvable from S to E"
            );
        }
    }

    #[test]
    fn test_small_maze_solvable() {
        let mut maze = CylinderMaze::new(3, 3);
        let (start, end) = maze.generate_wilson();

        assert!(
            maze.can_solve(start, end),
            "Small maze should be solvable from S to E"
        );
    }

    #[test]
    fn test_large_maze_solvable() {
        let mut maze = CylinderMaze::new(50, 50);
        let (start, end) = maze.generate_wilson();

        assert!(
            maze.can_solve(start, end),
            "Large maze should be solvable from S to E"
        );
    }

    #[test]
    fn test_unsolvable_maze() {
        // Create a maze with no path between start and end
        let maze = CylinderMaze::new(3, 3);
        let start = (0, 0);
        let end = (2, 2);

        // Don't generate paths - all walls
        // This maze should not be solvable
        assert!(
            !maze.can_solve(start, end),
            "Maze with all walls should not be solvable"
        );
    }
}
