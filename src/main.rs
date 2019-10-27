#[macro_use]
extern crate configure_me;

include_config!();

fn main() {
    // Don't worry, unwrap_or_exit() prints a nice message instead of ugly panic
    let (server_config, _remaining_args) = Config::including_optional_config_files(&["config.toml"]).unwrap_or_exit();
    dbg!(server_config.port);

    // Your code here
    // E.g.:
    // let listener = std::net::TcpListener::bind((server_config.bind_addr, server_config.port)).expect("Failed to bind socket");
}
//
//
// use quick_calc;
//
// fn main() {
//     quick_calc::run().unwrap();
// }
