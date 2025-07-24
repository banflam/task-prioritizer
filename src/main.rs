use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    due_date: Option<String>,
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn create_new_task() -> Option<Task> {
    let description = prompt("Enter task (or 'q' to quit): ");
    if description.eq_ignore_ascii_case("q") {
        return None;
    }

    let due = prompt("Enter due date (or leave blank): ");
    let due_date = if due.is_empty() { None } else { Some(due) };
    Some(Task { description, due_date })
}

fn insert_task_interactively(mut tasks: Vec<Task>, new_task: Task) -> Vec<Task> {
    println!("\nCurrent tasks:");
    for (i, task) in tasks.iter().enumerate() {
        let pos = i + 1;
        println!("{:>2}: {}", pos, task.description);
    }

    let mut pos = tasks.len();
    loop {
        let visible_pos = pos + 1;
        println!("\nNew task: {}", new_task.description);
        println!("Insert at position: {}", visible_pos);
        println!("Commands: (u)p, (d)own, (s)ave, (q)uit without saving");

        match prompt("> ").as_str() {
            "u" if pos > 0 => pos -= 1,
            "d" if pos < tasks.len() => pos += 1,
            "s" => break,
            "q" => return tasks, // discard new_task
            _ => println!("Invalid input"),
        }
    }

    tasks.insert(pos, new_task);
    tasks
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        match create_new_task() {
            Some(new_task) => {
                tasks = insert_task_interactively(tasks, new_task);
            }
            None => {
                println!("Exiting...");
                break;
            }
        }
    }

    save_tasks(&tasks);
    println!("✅ All tasks saved. Bye!");
}