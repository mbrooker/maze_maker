use crate::maze::{Cell, CylinderMaze};
use anyhow::Result;
use std::f64::consts::{PI, TAU};

use csgrs::{traits::CSG};

type Mesh = csgrs::mesh::Mesh<()>;
type Sketch = csgrs::sketch::Sketch<()>;

pub fn maze_to_stl(maze: &CylinderMaze, scale: f64, filename: &str) -> Result<()> {
    let radius = scale;
    let grid = maze.grid();
    let circ = TAU * scale;
    let seg_scale = circ / grid.len() as f64;

    let mut cylinder = Mesh::cylinder(radius, seg_scale * grid.len() as f64, 360, None);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Cell::Path {
                println!("Adding path seg {} {}", row, col);
                let angle_deg = 360.0 * col as f64 / grid[row].len() as f64;
                let cube = Mesh::cube(seg_scale * 1.01, None)
                    .translate(-seg_scale / 2.0, -seg_scale / 2.0, 0.0)
                    //.rotate(0.0, 0.0, angle_deg / 2.0)
                    .translate(radius - seg_scale * 0.45, 0.0, row as f64 * seg_scale)
                    .rotate(0.0, 0.0, angle_deg);
                cylinder = cylinder.difference(&cube);
            }
        }
    }
    println!("Path segs done, addding base.");

    let base =
        Mesh::cylinder(radius * 1.1, seg_scale, 1024, None).translate(0.0, 0.0, -seg_scale * 0.99);
    cylinder = base.union(&cylinder);
    println!("Generating STL");
    let stl = cylinder.to_stl_binary("my_solid")?;
    std::fs::write(filename, stl)?;
    Ok(())
}

pub fn make_outer_stl(scale: f64, height_cells: usize, filename: &str) -> Result<()> {
    let inner_radius = scale + 0.5;
    let outer_radius = scale + 2.0;
    let circ = TAU * scale;
    let seg_scale = circ / height_cells as f64;
    let height = height_cells as f64 * seg_scale;

    let outer_cylinder = Mesh::cylinder(outer_radius, height, 360, None);
    let inner_cylinder = Mesh::cylinder(inner_radius, height, 360, None);
    let base = Mesh::cylinder(outer_radius * 1.1, seg_scale, 360, None).translate(
        0.0,
        0.0,
        -seg_scale * 0.99,
    );
    let bottom_width = seg_scale*0.45;
    let top_width = seg_scale*0.375;
    let tooth = Sketch::trapezoid(top_width, bottom_width , seg_scale*0.6, 0.0, None)
        .translate(0.0, -bottom_width / 2.0, 0.0);
    let tooth_3d = tooth.revolve(360.0, 10).unwrap();//.translate(0.0, -inner_radius, height - bottom_width);


    let stl = outer_cylinder
        .difference(&inner_cylinder)
        .union(&base)
        .union(&tooth_3d)
        .to_stl_binary("outer_cyl")?;
    std::fs::write(filename, stl)?;

    let t_stl = tooth_3d.to_stl_binary("tooth")?;
    std::fs::write("tooth.stl", t_stl)?;
    Ok(())
}
