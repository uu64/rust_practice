use std::process;

use minigrep::parse_arguments;


fn main() {
    let config = parse_arguments();
    println!("{:?}", config);


    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
