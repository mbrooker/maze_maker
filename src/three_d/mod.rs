use crate::maze::{Cell, CylinderMaze};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;

pub struct CylinderMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl CylinderMesh {
    /// Generate a 3D cylindrical mesh from a CylinderMaze
    /// The maze wraps around the cylinder horizontally
    /// Walls are at the full outer diameter, paths are embossed inward
    pub fn from_maze(maze: &CylinderMaze, wall_height: f32) -> Self {
        let grid = maze.grid();
        let rows = grid.len();
        let cols = grid[0].len();

        // Calculate cylinder dimensions from maze
        let circumference = cols as f32;
        let outer_radius = circumference / (2.0 * PI);
        let inner_radius = outer_radius - wall_height;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate mesh by creating quads for each cell
        for row in 0..rows {
            for col in 0..cols {
                let cell = grid[row][col];

                // Calculate angular position
                let angle = (col as f32 / cols as f32) * 2.0 * PI;
                let next_angle = ((col + 1) as f32 / cols as f32) * 2.0 * PI;
                let y = row as f32;
                let y_next = (row + 1) as f32;

                // Choose radius based on cell type
                let radius = match cell {
                    Cell::Wall => outer_radius, // Walls at full diameter
                    Cell::Path => inner_radius, // Paths embossed inward
                };

                // Create quad vertices for this cell (horizontal surface)
                let base_idx = vertices.len() as u32;

                // Bottom-left
                let x0 = radius * angle.cos();
                let z0 = radius * angle.sin();
                vertices.push([x0, y, z0]);

                // Bottom-right
                let x1 = radius * next_angle.cos();
                let z1 = radius * next_angle.sin();
                vertices.push([x1, y, z1]);

                // Top-right
                let x2 = radius * next_angle.cos();
                let z2 = radius * next_angle.sin();
                vertices.push([x2, y_next, z2]);

                // Top-left
                let x3 = radius * angle.cos();
                let z3 = radius * angle.sin();
                vertices.push([x3, y_next, z3]);

                // Create two triangles for the quad
                indices.extend_from_slice(&[
                    base_idx,
                    base_idx + 1,
                    base_idx + 2,
                    base_idx,
                    base_idx + 2,
                    base_idx + 3,
                ]);

                // Add vertical walls at transitions between path and wall
                if cell == Cell::Path {
                    // Check right neighbor (wrapping around)
                    let next_col = (col + 1) % cols;
                    let right_cell = grid[row][next_col];

                    if right_cell == Cell::Wall {
                        // Create vertical wall on the right edge (at next_angle)
                        let wall_idx = vertices.len() as u32;

                        let x_inner = inner_radius * next_angle.cos();
                        let z_inner = inner_radius * next_angle.sin();
                        let x_outer = outer_radius * next_angle.cos();
                        let z_outer = outer_radius * next_angle.sin();

                        vertices.push([x_inner, y, z_inner]);
                        vertices.push([x_outer, y, z_outer]);
                        vertices.push([x_outer, y_next, z_outer]);
                        vertices.push([x_inner, y_next, z_inner]);

                        indices.extend_from_slice(&[
                            wall_idx,
                            wall_idx + 1,
                            wall_idx + 2,
                            wall_idx,
                            wall_idx + 2,
                            wall_idx + 3,
                        ]);
                    }

                    // Check left neighbor (wrapping around)
                    let prev_col = if col == 0 { cols - 1 } else { col - 1 };
                    let left_cell = grid[row][prev_col];

                    if left_cell == Cell::Wall {
                        // Create vertical wall on the left edge (at angle)
                        let wall_idx = vertices.len() as u32;

                        let x_inner = inner_radius * angle.cos();
                        let z_inner = inner_radius * angle.sin();
                        let x_outer = outer_radius * angle.cos();
                        let z_outer = outer_radius * angle.sin();

                        vertices.push([x_inner, y, z_inner]);
                        vertices.push([x_outer, y, z_outer]);
                        vertices.push([x_outer, y_next, z_outer]);
                        vertices.push([x_inner, y_next, z_inner]);

                        indices.extend_from_slice(&[
                            wall_idx,
                            wall_idx + 1,
                            wall_idx + 2,
                            wall_idx,
                            wall_idx + 2,
                            wall_idx + 3,
                        ]);
                    }

                    // Check top neighbor
                    if row > 0 {
                        let top_cell = grid[row - 1][col];

                        if top_cell == Cell::Wall {
                            // Create horizontal wall on the top edge (at y)
                            let wall_idx = vertices.len() as u32;

                            let x0_inner = inner_radius * angle.cos();
                            let z0_inner = inner_radius * angle.sin();
                            let x1_inner = inner_radius * next_angle.cos();
                            let z1_inner = inner_radius * next_angle.sin();
                            let x0_outer = outer_radius * angle.cos();
                            let z0_outer = outer_radius * angle.sin();
                            let x1_outer = outer_radius * next_angle.cos();
                            let z1_outer = outer_radius * next_angle.sin();

                            vertices.push([x0_inner, y, z0_inner]);
                            vertices.push([x1_inner, y, z1_inner]);
                            vertices.push([x1_outer, y, z1_outer]);
                            vertices.push([x0_outer, y, z0_outer]);

                            indices.extend_from_slice(&[
                                wall_idx,
                                wall_idx + 1,
                                wall_idx + 2,
                                wall_idx,
                                wall_idx + 2,
                                wall_idx + 3,
                            ]);
                        }
                    }

                    // Check bottom neighbor
                    if row < rows - 1 {
                        let bottom_cell = grid[row + 1][col];

                        if bottom_cell == Cell::Wall {
                            // Create horizontal wall on the bottom edge (at y_next)
                            let wall_idx = vertices.len() as u32;

                            let x0_inner = inner_radius * angle.cos();
                            let z0_inner = inner_radius * angle.sin();
                            let x1_inner = inner_radius * next_angle.cos();
                            let z1_inner = inner_radius * next_angle.sin();
                            let x0_outer = outer_radius * angle.cos();
                            let z0_outer = outer_radius * angle.sin();
                            let x1_outer = outer_radius * next_angle.cos();
                            let z1_outer = outer_radius * next_angle.sin();

                            vertices.push([x0_inner, y_next, z0_inner]);
                            vertices.push([x1_inner, y_next, z1_inner]);
                            vertices.push([x1_outer, y_next, z1_outer]);
                            vertices.push([x0_outer, y_next, z0_outer]);

                            indices.extend_from_slice(&[
                                wall_idx,
                                wall_idx + 1,
                                wall_idx + 2,
                                wall_idx,
                                wall_idx + 2,
                                wall_idx + 3,
                            ]);
                        }
                    }
                }
            }
        }

        CylinderMesh { vertices, indices }
    }

    /// Calculate cylinder dimensions from maze
    pub fn calculate_dimensions(maze: &CylinderMaze) -> (f32, f32) {
        let grid = maze.grid();
        let rows = grid.len();
        let cols = grid[0].len();

        let height = rows as f32;
        let circumference = cols as f32;
        let diameter = circumference / PI;

        (height, diameter)
    }

    /// Export the mesh to an STL file
    pub fn export_stl(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        // Convert indexed mesh to triangles for STL
        let mut triangles = Vec::new();

        for chunk in self.indices.chunks(3) {
            if chunk.len() == 3 {
                let v0 = self.vertices[chunk[0] as usize];
                let v1 = self.vertices[chunk[1] as usize];
                let v2 = self.vertices[chunk[2] as usize];

                // Calculate normal vector
                let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
                let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

                let normal = [
                    edge1[1] * edge2[2] - edge1[2] * edge2[1],
                    edge1[2] * edge2[0] - edge1[0] * edge2[2],
                    edge1[0] * edge2[1] - edge1[1] * edge2[0],
                ];

                // Normalize
                let length =
                    (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
                let normal = if length > 0.0 {
                    [normal[0] / length, normal[1] / length, normal[2] / length]
                } else {
                    [0.0, 0.0, 1.0]
                };

                triangles.push(stl_io::Triangle {
                    normal: stl_io::Vector::new(normal),
                    vertices: [
                        stl_io::Vector::new(v0),
                        stl_io::Vector::new(v1),
                        stl_io::Vector::new(v2),
                    ],
                });
            }
        }

        stl_io::write_stl(&mut writer, triangles.iter())?;
        Ok(())
    }
}
