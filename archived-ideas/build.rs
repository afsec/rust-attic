#![warn(clippy::all)]

use std::env;
use std::io;
use std::path::Path;

fn copy(input_fname: &str, output_fname: &str) -> io::Result<()> {
    use std::fs;
    use std::io::{Read, Write};

    let mut input_file = fs::File::open(input_fname)?;
    let mut contents = Vec::new();

    input_file.read_to_end(&mut contents)?;

    let mut output_file = fs::File::create(output_fname)?;

    output_file.write_all(&contents)
}

fn main() -> io::Result<()> {
    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let pkg_name = env::var_os("CARGO_PKG_NAME").unwrap();

    let mut input_path = Path::new(&out_dir).join(&pkg_name);
    input_path.set_extension("wasm");

    println!("cargo:warning=DEBUG: {:?}", input_path);

    let dst = "";

    copy(input_path.to_str().unwrap(), dst)?;

    Ok(())
}
