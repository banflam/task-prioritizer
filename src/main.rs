use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor:MoveTo,
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout};

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
    let mut pos: usize = 0;
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        // Header
        println!("Use 'j' (down), 'k' (up), Enter to insert:");

        // Draw task list with manual cursor moves
        for (i, task) in tasks.iter().enumerate() {
            let y = (i + 1) as u16 + 1; // +1 for header
            let prefix = if i == pos { "> " } else { "  " };
            execute!(stdout, MoveTo(0, y)).unwrap();
            write!(stdout, "{}{}", prefix, task.description).unwrap();
        }

        // Show the "[insert at end]" line
        let y = (tasks.len() + 1) as u16 + 1;
        execute!(stdout, MoveTo(0, y)).unwrap();
        let prefix = if pos == tasks.len() { "> " } else { "  " };
        write!(stdout, "{}[insert at end]", prefix).unwrap();

        stdout.flush().unwrap();

        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('j') if pos < tasks.len() => pos += 1,
                KeyCode::Char('k') if pos > 0 => pos -= 1,
                KeyCode::Enter => break,
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    return tasks;
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
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