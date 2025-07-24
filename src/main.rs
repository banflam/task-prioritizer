use serde::{Serialize, Deserialize}
use std::fs;

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or(vec![])
}

struct Task {
    name: String,
    due_date: String,
    importance: u8,
}

fn main() {
    println!("Hello, world!");
}
