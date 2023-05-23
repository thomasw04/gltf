use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    let json = gltf.document.into_json().to_string_pretty()?;
    println!("{}", json);
    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-roundtrip <FILE>");
    }
}
