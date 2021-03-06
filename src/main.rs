use recuring_task_manager::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = recuring_task_manager::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

//rtm new daily "dsds"
//ret del daily "dsds"
