use crate::maze::{Cell, CylinderMaze};
use anyhow::Result;
use std::f64::consts::TAU;

/// Generate OpenSCAD code for the maze cylinder
pub fn maze_to_openscad(
    maze: &CylinderMaze,
    height: f64,
    circumference: f64,
    filename: &str,
    hollow: bool,
) -> Result<()> {
    let radius = circumference / TAU;
    let grid = maze.grid();

    let seg_scale_x = circumference / grid[0].len() as f64;
    let seg_scale_z = height / grid.len() as f64;
    let height = seg_scale_z * grid.len() as f64;

    let mut scad = String::new();

    // Define parameters
    scad.push_str(&format!("radius = {radius};\n"));
    scad.push_str(&format!("seg_scale_x = {seg_scale_x};\n"));
    scad.push_str(&format!("seg_scale_z = {seg_scale_z};\n"));
    scad.push_str(&format!("height = {height};\n"));
    scad.push_str(&format!("rows = {};\n", grid.len()));
    scad.push_str(&format!("cols = {};\n", grid[0].len()));
    scad.push('\n');

    // Build maze data array - collect path cells
    scad.push_str("// Maze data: [row, col] pairs for path cells\n");
    scad.push_str("maze_paths = [\n");
    for (row, row_cells) in grid.iter().enumerate() {
        for (col, cell) in row_cells.iter().enumerate() {
            if *cell == Cell::Path {
                scad.push_str(&format!("  [{row}, {col}],\n"));
            }
        }
    }
    scad.push_str("];\n\n");

    // Generate the maze using OpenSCAD for loop
    scad.push_str("union() {\n");
    scad.push_str("  difference() {\n");
    scad.push_str("    cylinder(r=radius, h=height, $fn=360);\n");
    scad.push_str("    \n");
    scad.push_str("    // Carve out path segments\n");
    scad.push_str("    for (path = maze_paths) {\n");
    scad.push_str("      row = path[0];\n");
    scad.push_str("      col = path[1];\n");
    scad.push_str("      angle = 360 * col / cols;\n");
    scad.push_str("      z_pos = row * seg_scale_z;\n");
    scad.push_str("      \n");
    scad.push_str("      rotate([0, 0, angle])\n");
    scad.push_str("        translate([radius - seg_scale_x * 0.45, -seg_scale_x / 2, z_pos])\n");
    scad.push_str("          cube([seg_scale_x * 1.01, seg_scale_x, seg_scale_z * 1.01]);\n");
    scad.push_str("    }\n");
    if hollow {
        scad.push_str("    cylinder(r=radius-seg_scale_x, h=height+0.1, $fn=360);\n");
    }
    scad.push_str("  }\n");
    scad.push_str("  \n");
    scad.push_str("  // Base\n");
    scad.push_str("  translate([0, 0, -height * 0.05])\n");
    scad.push_str("    cylinder(r=radius * 1.1, h=height * 0.05, $fn=360);\n");
    scad.push_str("}\n");

    // Write the whole model
    std::fs::write(format!("{filename}_whole.scad"), &scad)?;

    Ok(())
}

/// Generate OpenSCAD code for the outer cylinder
pub fn make_outer_openscad(
    height: f64,
    circumference: f64,
    rows: usize,
    cols: usize,
    filename: &str,
) -> Result<()> {
    let radius = circumference / TAU;
    let inner_radius = radius + 0.2;
    let outer_radius = (radius * 1.1).max(inner_radius + 1.2);

    let seg_scale_x = circumference / cols as f64;
    let seg_scale_z = height / rows as f64;

    let mut scad = String::new();

    // Define parameters
    scad.push_str(&format!("inner_radius = {inner_radius};\n"));
    scad.push_str(&format!("outer_radius = {outer_radius};\n"));
    scad.push_str(&format!("height = {height};\n"));
    scad.push_str(&format!("seg_scale_x = {seg_scale_x};\n"));
    scad.push_str(&format!("seg_scale_z = {seg_scale_z};\n"));
    scad.push('\n');

    scad.push_str("union() {\n");

    // Hollow cylinder (outer - inner)
    scad.push_str("  difference() {\n");
    scad.push_str("    cylinder(r=outer_radius, h=height, $fn=360);\n");
    scad.push_str("    cylinder(r=inner_radius, h=height * 1.01, $fn=360);\n");
    scad.push_str("  }\n");

    // Base
    scad.push_str("  translate([0, 0, -height * 0.05])\n");
    scad.push_str("    cylinder(r=outer_radius * 1.1, h=height * 0.05, $fn=360);\n");

    // Tooth on outer wall at top
    scad.push_str("  // Tooth on outer wall at top\n");
    scad.push_str(
        "  translate([- outer_radius + seg_scale_x * 0.35, 0, height - seg_scale_z * 0.45])\n",
    );
    scad.push_str("   scale([seg_scale_x, seg_scale_x, seg_scale_z])\n");
    scad.push_str("    rotate([0, 90, 0])\n");
    scad.push_str("      cylinder(r1=0.30, r2=0.3 * 0.8, h=0.30, $fn=36);\n");

    scad.push_str("}\n");

    std::fs::write(format!("{filename}.scad"), scad)?;

    Ok(())
}
