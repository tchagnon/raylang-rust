use std::env;

mod math;
mod mesh;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <file.smf>");
        return;
    }

    let path = &args[1];
    let mesh = mesh::read_smf(path);
    println!("vertices: {} faces: {}", mesh.vertices.len(), mesh.faces.len());
}
