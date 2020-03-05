use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();


    let config = Config::new(&args).unwrap_or_else( |error| {
        println!("shit! {}", error);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("motherfucker!, cannot open file {}", e);
        process::exit(1);
    }
}

