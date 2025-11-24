mod maze;
mod three_d;

use anyhow::Result;
use clap::Parser;
use maze::CylinderMaze;
use three_d::{make_outer_openscad, maze_to_openscad};

#[derive(Parser, Debug)]
#[command(name = "maze_maker")]
#[command(about = "Generate cylindrical mazes and export to OpenSCAD", long_about = None)]
struct Args {
    /// Number of rows in the maze
    #[arg(short, long, default_value_t = 10)]
    rows: usize,

    /// Number of columns in the maze
    #[arg(short, long, default_value_t = 20)]
    cols: usize,

    /// Height of the cylinder
    #[arg(long, default_value_t = 30.0)]
    height: f64,

    /// Circumference of the cylinder
    #[arg(long, default_value_t = 60.0)]
    circumference: f64,

    /// Base filename for the maze output
    #[arg(long, default_value = "cylinder_maze")]
    maze_file: String,

    /// Base filename for the outer cylinder output
    #[arg(long, default_value = "cylinder_outer")]
    outer_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut maze = CylinderMaze::new(args.rows, args.cols);
    let (start, end) = maze.generate_wilson();

    println!(
        "Wilson's Algorithm Maze on a Cylinder ({}x{}):",
        args.rows, args.cols
    );
    println!("(Left and right edges wrap around)");
    println!("Start (S) at top row, End (E) at bottom row\n");
    maze.display(start, end);

    println!("\nMaze is solvable: {}", maze.can_solve(start, end));
    maze_to_openscad(&maze, args.height, args.circumference, &args.maze_file)?;
    make_outer_openscad(args.height, args.circumference, args.rows, args.cols, &args.outer_file)?;
    Ok(())
}
