use std::process::exit;
use std::env::args;
use vapor_cli::Config;
use vapor_cli::run;

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Appliction Error : {}", e);
        exit(1);
    }
}