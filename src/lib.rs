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
struct Task {
    id: u16,
    name: String
}

impl Task {
    pub fn new(id: u16, name:String) -> Task {
        Task {
            id,
            name
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    daily: Vec<Task>,
    weekly: Vec<Task>,
    monthly: Vec<Task>
}

pub fn run(config: Config) -> Result<(), &'static str> {
    match config.command.as_str() {
        "add" => {
            add_new_task(config);
        },
        "del" => {
            delete_task(config);
        },
        _ => return Err("Invalid command"),
    }

    Ok(())
}

fn get_task_data() -> Tasks {
    let data = match fs::read_to_string("tasks.json") {
        Ok(s) => s,
        Err(_e) => String::from("{ \"daily\": [], \"weekly\": [], \"monthly\": [] }"),
    };

    serde_json::from_str(data.as_str()).unwrap()
}

fn get_json_recurrence<'a>(string: &str, json: &'a mut Tasks) -> &'a mut Vec<Task> {
    let recurrence = match string {
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

    recurrence
}

fn generate_id(tasks: &Vec<Task>) -> u16 {
    let default_task = Task::new(0, String::new());
    tasks.last().unwrap_or(&default_task).id + 1
}

fn update_json(new_json: &Tasks) {
    let updated_json = serde_json::to_string(new_json).unwrap();
    fs::write("tasks.json", updated_json).expect("Unable to write file");
}

fn add_new_task(config: Config) {
    let mut json = get_task_data();
    let recurrence = get_json_recurrence(config.recurrence.as_str(), &mut json);

    let id = generate_id(recurrence);
    let name = config.name;
    let new_task = Task::new(id, name);
    recurrence.push(new_task);

    update_json(&json);
}

fn delete_task(config: Config) {
    let mut json = get_task_data();
    let recurrence = get_json_recurrence(config.recurrence.as_str(), &mut json);
   
    let name = config.name;
    let task_index = recurrence.iter().position(|x| *x.name == name).unwrap();
    recurrence.remove(task_index);

    update_json(&json);
}