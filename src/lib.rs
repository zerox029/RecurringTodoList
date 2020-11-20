use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs};

pub struct Config {
    pub command: String,
    pub name: String,
    pub recurrence: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments supplied");
        }

        let command = args[1].clone();
        let recurrence = args[2].clone();
        let name = args[3].clone();

        Ok(Config { command, recurrence, name })
    }
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    daily: Vec<String>,
    weekly: Vec<String>,
    monthly: Vec<String>
}

pub fn run(config: Config) -> Result<(), &'static str> {
    match config.command.as_str() {
        "add" => {
            add_new_task(config);
        }
        _ => return Err("Invalid command"),
    }

    Ok(())
}

pub fn add_new_task(config: Config) {
    let data = match fs::read_to_string("tasks.json") {
        Ok(s) => s,
        Err(_e) => String::from("{ \"daily\": [], \"weekly\": [], \"monthly\": [] }"),
    };
    let mut json: Tasks = serde_json::from_str(data.as_str()).unwrap();

    let recurrence = match config.recurrence.as_str() {
        "daily" => &mut json.daily,
        "day" => &mut json.daily,
        "d" => &mut json.daily,
        "weekly" => &mut json.weekly,
        "week" => &mut json.weekly,
        "w" => &mut json.weekly,
        "monthly" => &mut json.monthly,
        "month" => &mut json.monthly,
        "m" => &mut json.monthly,
        _ => &mut json.daily
    };

    recurrence.push(config.name);

    let updated_json = serde_json::to_string(&json).unwrap();
    fs::write("tasks.json", updated_json).expect("Unable to write file");
}
