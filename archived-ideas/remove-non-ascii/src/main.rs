fn main() -> anyhow::Result<()> {
    use std::env;
    use std::fs;
    use std::io::Write;

    // --snip--
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./{} <filename>", env!("CARGO_PKG_NAME"));
    } else {
        let filename = &args[1];

        let contents = fs::read_to_string(filename)?;
        let output: Vec<String> = contents
            .lines()
            .into_iter()
            .map(|line| line.replace(|c: char| !c.is_ascii(), ""))
            .collect();

        let buf = output.join("\n");
        let mut file = fs::File::create(filename)?;

        file.write_all(buf.as_bytes())?;
    }
    Ok(())
}
