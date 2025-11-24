use crate::maze::{Cell, CylinderMaze};
use anyhow::Result;
use std::f64::consts::TAU;

/// Generate OpenSCAD code for the maze cylinder
pub fn maze_to_openscad(maze: &CylinderMaze, height: f64, circumfernce: f64, filename: &str) -> Result<()> {
    let radius = circumfernce / TAU;
    let grid = maze.grid();

    let seg_scale_x = circumfernce / grid[0].len() as f64;
    let seg_scale_z = height / grid.len() as f64;
    let height = seg_scale_z * grid.len() as f64;

    let mut scad = String::new();
    
    // Start with difference operation
    scad.push_str("difference() {\n");
    
    // Main cylinder
    scad.push_str(&format!("  cylinder(r={}, h={}, $fn=360);\n", radius, height));
    
    // Subtract path segments
    scad.push_str("  union() {\n");
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Cell::Path {
                let angle_deg = 360.0 * col as f64 / grid[row].len() as f64;
                let z_pos = row as f64 * seg_scale_z;
                
                scad.push_str(&format!(
                    "    rotate([0, 0, {}]) translate([{}, {}, {}]) cube([{}, {}, {}], center=false);\n",
                    angle_deg,
                    radius - seg_scale_x * 0.45,
                    -seg_scale_x / 2.0,
                    z_pos,
                    seg_scale_x * 1.01,
                    seg_scale_x,
                    seg_scale_z * 1.01
                ));
            }
        }
    }
    scad.push_str("  }\n");
    scad.push_str("}\n");

    // Write the whole model
    std::fs::write(format!("{}_whole.scad", filename), &scad)?;

    Ok(())
}

/// Generate OpenSCAD code for the outer cylinder
pub fn make_outer_openscad(scale: f64, height_cells: usize, filename: &str) -> Result<()> {
    let inner_radius = scale + 0.5;
    let outer_radius = scale + 2.0;
    let circ = TAU * scale;
    let seg_scale = circ / height_cells as f64;
    let height = height_cells as f64 * seg_scale;

    let mut scad = String::new();
    
    scad.push_str("union() {\n");
    
    // Hollow cylinder (outer - inner)
    scad.push_str("  difference() {\n");
    scad.push_str(&format!("    cylinder(r={}, h={}, $fn=360);\n", outer_radius, height));
    scad.push_str(&format!("    cylinder(r={}, h={}, $fn=360);\n", inner_radius, height));
    scad.push_str("  }\n");
    
    // Base
    scad.push_str(&format!(
        "  translate([0, 0, {}]) cylinder(r={}, h={}, $fn=360);\n",
        -seg_scale * 0.99,
        outer_radius * 1.1,
        seg_scale
    ));
    
    scad.push_str("}\n");

    std::fs::write(format!("{}.scad", filename), scad)?;

    Ok(())
}
