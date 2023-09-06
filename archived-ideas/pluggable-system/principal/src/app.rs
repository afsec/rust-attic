use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use std::path::Path;

#[derive(WrapperApi)]
struct PluginsApi {
    // * Função para entrypoint
    presenter: extern "C" fn(input: Option<String>),

    // * Função para metadados
    // * Mostrar: versão, nome do pacote
    help: extern "C" fn(),
}

fn load_plugins() {
    // TODO: Add error handling
    let files_in_dir = std::fs::read_dir("./plugins").unwrap();

    for entry in files_in_dir {
        // TODO: Add error handling
        let path: std::path::PathBuf = entry.unwrap().path();

        // TODO: Add error handling
        let path_str = path.to_str().unwrap();

        let current_plugin: Container<PluginsApi> = unsafe {
            Container::load(Path::new(&path_str)).expect("Could not open library or load symbols")
        };

        println!("----");
        println!("RUNNING {}", path_str);

        current_plugin.help();
        current_plugin.presenter(Some("La bolicha".to_string()));
        current_plugin.presenter(None);
    }
}

pub fn run() {
    let package_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("Package Name: {}", package_name);
    println!("Version: {}", version);

    load_plugins();
}
