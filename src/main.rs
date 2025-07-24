use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{terminal, ExecutableCommand};
use std::io::{stdout};
use std::fmt::Write as FmtWrite;
use std::fmt::Write as FmtWrite; // For write!() on String

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
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();

    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(crossterm::cursor::MoveTo(0, 0)).unwrap();

        let mut buffer = String::new();
        writeln!(buffer, "Use 'j' (down), 'k' (up), Enter to insert:").unwrap();

        for (i, task) in tasks.iter().enumerate() {
            if i == pos {
                writeln!(buffer, "> {}", task.description).unwrap();
            } else {
                writeln!(buffer, "  {}", task.description).unwrap();
            }
        }

        if pos == tasks.len() {
            writeln!(buffer, "> [insert at end]").unwrap();
        } else {
            writeln!(buffer, "  [insert at end]").unwrap();
        }

        write!(stdout, "{}", buffer).unwrap();
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