use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <file.smf>");
        return;
    }

    let path = &args[1];
    let smf_file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Could not open \"{}\": {}", path, why),
    };
    let input = BufReader::new(smf_file);

    let lines   : Vec<_> = input.lines().map(|l| l.unwrap()).collect();
    let vertices: Vec<_> = lines.iter().filter(|l| l.starts_with("v")).collect();
    let faces   : Vec<_> = lines.iter().filter(|l| l.starts_with("f")).collect();
    println!("vertices: {} faces: {}", vertices.len(), faces.len());
}
