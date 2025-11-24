mod maze;
mod three_d;

use maze::CylinderMaze;
use three_d::CylinderMesh;

fn main() {
    let rows = 10;
    let cols = 10;

    let mut maze = CylinderMaze::new(rows, cols);
    let (start, end) = maze.generate_wilson();

    println!("Wilson's Algorithm Maze on a Cylinder ({}x{}):", rows, cols);
    println!("(Left and right edges wrap around)");
    println!("Start (S) at top row, End (E) at bottom row\n");
    maze.display(start, end);

    println!("\nMaze is solvable: {}", maze.can_solve(start, end));

    // Generate 3D cylinder mesh
    let (height, diameter) = CylinderMesh::calculate_dimensions(&maze);
    println!("\n3D Cylinder Dimensions:");
    println!("  Height: {:.2}", height);
    println!("  Diameter: {:.2}", diameter);

    let wall_height = 0.5;
    let mesh = CylinderMesh::from_maze(&maze, wall_height);
    println!("\n3D Maze Mesh Generated:");
    println!("  Vertices: {}", mesh.vertices.len());
    println!("  Triangles: {}", mesh.indices.len() / 3);

    // Export maze to STL file
    let filename = "cylinder_maze.stl";
    match mesh.export_stl(filename) {
        Ok(_) => println!("\nMaze STL file exported successfully: {}", filename),
        Err(e) => eprintln!("\nError exporting maze STL: {}", e),
    }

    // Generate outer cylinder shell
    let wall_thickness = 1.0;
    let outer_mesh = CylinderMesh::outer_cylinder(&maze, wall_height, wall_thickness);
    println!("\n3D Outer Cylinder Mesh Generated:");
    println!("  Vertices: {}", outer_mesh.vertices.len());
    println!("  Triangles: {}", outer_mesh.indices.len() / 3);

    // Export outer cylinder to STL file
    let outer_filename = "cylinder_outer.stl";
    match outer_mesh.export_stl(outer_filename) {
        Ok(_) => println!("\nOuter cylinder STL file exported successfully: {}", outer_filename),
        Err(e) => eprintln!("\nError exporting outer cylinder STL: {}", e),
    }
}
