mod maze;
mod three_d;

use anyhow::Result;
use maze::CylinderMaze;
use three_d::maze_to_stl;

fn main() -> Result<()> {
    let rows = 10;
    let cols = 10;

    let mut maze = CylinderMaze::new(rows, cols);
    let (start, end) = maze.generate_wilson();

    println!("Wilson's Algorithm Maze on a Cylinder ({}x{}):", rows, cols);
    println!("(Left and right edges wrap around)");
    println!("Start (S) at top row, End (E) at bottom row\n");
    maze.display(start, end);

    println!("\nMaze is solvable: {}", maze.can_solve(start, end));
    maze_to_stl(&maze, 1.0, "cylinder_maze.stl")?;

    Ok(())
}
