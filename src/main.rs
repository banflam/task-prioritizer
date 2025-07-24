use serde::{Serialize, Deserialize}
use std::fs;
use std::io::{self, Write};

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or(vec![])
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn create_new_task() -> Task {
    let description = prompt("Enter task: ");
    let due = prompt("Enter due date (or leave blank): ");
    let due_date = if due.is_empty() { None } else { Some(due) };
    let importance: u8 = prompt("Importance (1-5): ").parse().unwrap_or(3);
    Task { description, due_date, importance }
}


fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

struct Task {
    name: String,
    due_date: Option<String>,
    importance: u8,
}

fn main() {
    println!("Hello, world!");
}
