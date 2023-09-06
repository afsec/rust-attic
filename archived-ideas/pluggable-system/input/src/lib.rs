#[derive(Debug)]
struct Package {
    name: &'static str,
    version: &'static str,
    data: Option<Vec<u8>>,
}

impl Package {
    fn new() -> Self {
        Package {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            data: None,
        }
    }
}

#[no_mangle]
extern "C" fn help() {
    let package = Package::new();
    println!("HELP: {}: {}",package.name, package.version);
}

#[no_mangle]
extern "C" fn run() {
    let package = Package::new();
    println!("RUN:");
    dbg!(package);
}
