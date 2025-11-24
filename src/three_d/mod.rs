use crate::maze::{Cell, CylinderMaze};
use anyhow::Result;
use std::f64::consts::{PI, TAU};

use csgrs::traits::CSG;

type Mesh = csgrs::mesh::Mesh<()>;

pub fn maze_to_stl(maze: &CylinderMaze, scale: f64, filename: &str) -> Result<()> {
    let radius = scale;
    let grid = maze.grid();
    let circ = TAU * scale;
    let seg_scale = circ / grid.len() as f64;

    let mut cylinder = Mesh::cylinder(radius, seg_scale * grid.len() as f64, 1024, None);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Cell::Path {
                println!("Adding path seg {} {}", row, col);
                let angle_deg = 360.0 * col as f64 / grid[row].len() as f64;
                let cube = Mesh::cube(seg_scale, None)
                    .rotate(0.0, 0.0, angle_deg / 2.0)
                    .translate(radius - seg_scale * 0.95, 0.0, row as f64 * seg_scale)
                    .rotate(0.0, 0.0, angle_deg);
                cylinder = cylinder.union(&cube);
            }
        }
    }
    println!("Path segs done, addding base.");

    let base = Mesh::cylinder(radius * 1.1, seg_scale, 1024, None).translate(0.0, 0.0, -seg_scale);
    //cylinder = cylinder.union(&base);
    println!("Generating STL");
    let stl = cylinder.to_stl_binary("my_solid")?;
    std::fs::write(filename, stl)?;
    Ok(())
}
