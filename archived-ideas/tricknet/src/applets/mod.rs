mod client;
mod server;
mod tricknet;

use crate::PKG_NAME;

use crate::errors::TrickError;

// #[derive(Debug)]
// struct Applet(String);

pub fn dispatcher() -> Result<(), TrickError> {
    let first_arg = std::env::args().nth(0).expect("no pattern given");
    let vec: Vec<&str> = first_arg.split("/").collect();

    let parsed_applet_name = if vec.len() == 1 {
        vec[0]
    } else {
        vec.last().unwrap()
    };

    match parsed_applet_name {
        PKG_NAME => tricknet::start(),
        "client" => client::start(),
        "server" => server::start(),
        "help" => tricknet::help(),
        _ => tricknet::usage(),
    }
}
