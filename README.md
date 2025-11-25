# Maze Maker

A Rust-based cylindrical maze generator that creates 3D-printable maze puzzles. The program generates mazes using Wilson's algorithm and exports them as OpenSCAD files, which can be converted to STL format for 3D printing.

## Features

- Generates perfect mazes on cylindrical surfaces using Wilson's algorithm
- Creates two separate components: an inner maze cylinder and an outer shell
- Wraps around horizontally (left and right edges connect)
- Exports to OpenSCAD format for easy 3D printing preparation
- Configurable maze dimensions and physical size
- Optional hollow interior for container-style mazes

## Installation

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).

Clone the repository and build:

```bash
git clone <repository-url>
cd maze_maker
cargo build --release
```

## Usage

Run the program with default settings:

```bash
cargo run --release
```

### Command Line Arguments

- `-r, --rows <ROWS>` - Number of rows in the maze (default: 10)
- `-c, --cols <COLS>` - Number of columns in the maze (default: 20)
- `--height <HEIGHT>` - Height of the cylinder in mm (default: 60.0)
- `--circumference <CIRCUMFERENCE>` - Circumference of the cylinder in mm (default: 100.0)
- `--maze-file <MAZE_FILE>` - Base filename for maze output (default: "cylinder_maze")
- `--outer-file <OUTER_FILE>` - Base filename for outer cylinder output (default: "cylinder_outer")
- `--hollow` - Hollow out the inside of the cylinder to make a container

### Examples

Generate a small maze:
```bash
cargo run --release -- --rows 8 --cols 16 --height 50 --circumference 80
```

Generate a large hollow maze container:
```bash
cargo run --release -- --rows 15 --cols 30 --height 80 --circumference 120 --hollow
```

Custom output filenames:
```bash
cargo run --release -- --maze-file my_maze --outer-file my_outer
```

## Converting to STL for 3D Printing

The program generates OpenSCAD files (`.scad`), which need to be converted to STL format for 3D printing.

### Using OpenSCAD GUI

1. Download and install [OpenSCAD](https://openscad.org/downloads.html)
2. Open the generated `.scad` file in OpenSCAD
3. Press F6 to render
4. Go to **File → Export → Export as STL**
5. Save the STL file

### Using OpenSCAD Command Line

For batch processing or automation:

```bash
openscad -o cylinder_maze_whole.stl cylinder_maze_whole.scad
openscad -o cylinder_outer.stl cylinder_outer.scad
```

On Windows:
```cmd
"C:\Program Files\OpenSCAD\openscad.exe" -o cylinder_maze_whole.stl cylinder_maze_whole.scad
"C:\Program Files\OpenSCAD\openscad.exe" -o cylinder_outer.stl cylinder_outer.scad
```

## Output Files

The program generates two OpenSCAD files:

- `<maze-file>_whole.scad` - The inner maze cylinder with carved paths
- `<outer-file>.scad` - The outer shell that fits around the maze

## How It Works

1. Generates a perfect maze using Wilson's loop-erased random walk algorithm
2. Maps the maze onto a cylindrical surface
3. Creates OpenSCAD code that:
   - Builds a solid cylinder
   - Carves out the maze paths
   - Adds a base platform for stability
4. Generates a matching outer shell with proper clearance

