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
    /// Walls are extruded outward from the cylinder surface
    pub fn from_maze(maze: &CylinderMaze, wall_height: f32) -> Self {
        let grid = maze.grid();
        let rows = grid.len();
        let cols = grid[0].len();

        // Calculate cylinder dimensions from maze
        let circumference = cols as f32;
        let radius = circumference / (2.0 * PI);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices for each cell in the maze grid
        for row in 0..rows {
            for col in 0..cols {
                let cell = grid[row][col];

                // Calculate position on cylinder
                let angle = (col as f32 / cols as f32) * 2.0 * PI;
                let y = row as f32;

                match cell {
                    Cell::Wall => {
                        // Create extruded wall geometry
                        let base_idx = vertices.len() as u32;

                        // Inner surface (at cylinder radius)
                        let x_inner = radius * angle.cos();
                        let z_inner = radius * angle.sin();
                        vertices.push([x_inner, y, z_inner]);

                        // Outer surface (extruded)
                        let outer_radius = radius + wall_height;
                        let x_outer = outer_radius * angle.cos();
                        let z_outer = outer_radius * angle.sin();
                        vertices.push([x_outer, y, z_outer]);

                        // Create quad faces for walls
                        if col < cols - 1 {
                            let next_col = col + 1;
                            let next_angle = (next_col as f32 / cols as f32) * 2.0 * PI;

                            // Add vertices for next column
                            let x_inner_next = radius * next_angle.cos();
                            let z_inner_next = radius * next_angle.sin();
                            vertices.push([x_inner_next, y, z_inner_next]);

                            let x_outer_next = outer_radius * next_angle.cos();
                            let z_outer_next = outer_radius * next_angle.sin();
                            vertices.push([x_outer_next, y, z_outer_next]);

                            // Outer face (two triangles)
                            indices.extend_from_slice(&[
                                base_idx + 1,
                                base_idx + 3,
                                base_idx + 2,
                                base_idx + 1,
                                base_idx + 2,
                                base_idx,
                            ]);
                        }

                        // Connect top and bottom if adjacent rows are also walls
                        if row < rows - 1 && grid[row + 1][col] == Cell::Wall {
                            let next_row_base = vertices.len() as u32;

                            let y_next = (row + 1) as f32;
                            vertices.push([x_inner, y_next, z_inner]);
                            vertices.push([x_outer, y_next, z_outer]);

                            // Side faces
                            indices.extend_from_slice(&[
                                base_idx + 1,
                                next_row_base + 1,
                                next_row_base,
                                base_idx + 1,
                                next_row_base,
                                base_idx,
                            ]);
                        }
                    }
                    Cell::Path => {
                        // Paths are just the cylinder surface (no extrusion)
                        let x = radius * angle.cos();
                        let z = radius * angle.sin();
                        vertices.push([x, y, z]);
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
                let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
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
