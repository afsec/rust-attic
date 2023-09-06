use crate::{errors::TrickError, PKG_NAME};

pub fn start() -> Result<(), TrickError> {
    if std::env::args().len() > 1 {
        let first_arg = std::env::args().nth(1).expect("no pattern given");
        let vec: Vec<&str> = first_arg.split("/").collect();
        let parsed_applet_name = if vec.len() == 1 {
            vec[0]
        } else {
            vec.last().unwrap()
        };
        match parsed_applet_name {
            "client" => super::client::start(),
            "server" => super::server::start(),
            "help" => help(),
            _ => panic!(),
        }
    } else {
        usage()
    }
}

pub fn usage() -> Result<(), TrickError> {
    print!(
        "{pkg_name} v{version}\n",
        pkg_name = PKG_NAME,
        version = env!("CARGO_PKG_VERSION")
    );

    println!("Usage: {pkg_name} client", pkg_name = PKG_NAME);
    println!("       {pkg_name} server", pkg_name = PKG_NAME);
    println!("       {pkg_name} help", pkg_name = PKG_NAME);
    Err(TrickError::InvalidCommand)
}

pub fn help() -> Result<(), TrickError> {
    print!(
        "HELP {pkg_name} v{version}\n\n",
        pkg_name = PKG_NAME,
        version = env!("CARGO_PKG_VERSION")
    );

    print!("Author: {author}\n\n", author = env!("CARGO_PKG_AUTHORS"));

    print!(
        "Description: {description}\n\n",
        description = env!("CARGO_PKG_DESCRIPTION")
    );

    println!("Usage: {pkg_name} client", pkg_name = PKG_NAME);
    println!("       {pkg_name} server", pkg_name = PKG_NAME);
    println!("       {pkg_name} help", pkg_name = PKG_NAME);
    Ok(())
}
