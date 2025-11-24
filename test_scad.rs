use scad::*;

fn main() {
    let mut sfile = ScadFile::new();
    
    // Try basic cylinder
    let cyl = scad!(Cylinder(10.0, Radius(5.0)));
    sfile.add_object(cyl);
    
    println!("{}", sfile);
}
