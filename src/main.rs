mod maze;
use maze::CylinderMaze;

fn main() {
    let rows = 20;
    let cols = 40;

    let mut maze = CylinderMaze::new(rows, cols);
    let (start, end) = maze.generate_wilson();

    println!("Wilson's Algorithm Maze on a Cylinder ({}x{}):", rows, cols);
    println!("(Left and right edges wrap around)");
    println!("Start (S) at top row, End (E) at bottom row\n");
    maze.display(start, end);

    println!("\nMaze is solvable: {}", maze.can_solve(start, end));
}

