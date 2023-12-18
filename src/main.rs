use rand::Rng;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Debug, Eq, PartialEq, Hash, Serialize)]
struct Task {
    name: String,
    status: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_string = format!("{}", self.name);
        write!(f, "{}", display_string)
    }
}

fn convert_string_to_number(str: &String) -> u8 {
    match str.trim().parse() {
        Ok(num) => return num,
        Err(_) => return 0,
    };
}

fn add_task(tasks: &mut HashMap<u8, Task>) {
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let task = Task {
        name: String::from(name),
        status: "Open".to_string(),
    };
    let mut rng = rand::thread_rng();
    let id: u8 = rng.gen();
    tasks.insert(id, task);
    println!("Task added");
    if let Err(err) = save_task_map_to_json_file(&tasks, "tasks.json") {
        eprintln!("Error: {}", err);
    }
}

fn complete_task(tasks: &mut HashMap<u8, Task>) {
    show_open_tasks(tasks);
    println!("Enter the task id: ");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");
    let task_id_number = task_id.trim().parse::<u8>();

    if let Ok(task_id_number) = task_id_number {
        if let Some(task) = tasks.get_mut(&task_id_number) {
            task.status = "Closed".to_string();
            if let Err(err) = save_task_map_to_json_file(tasks, "tasks.json") {
                eprintln!("Error: {}", err);
            }
        } else {
            println!("Id not found");
        }
    } else {
        println!("Invalid");
    }
}

fn show_open_tasks(tasks: &HashMap<u8, Task>) {
    for (&id, task) in tasks {
        if task.status == "Open" {
            println!(
                "Task ID: {} | Name: {} | Status: {}",
                id, task.name, task.status
            );
        }
    }
}

fn delete_task(tasks: &mut HashMap<u8, Task>) {
    show_open_tasks(&tasks);
    println!("Enter the task id: ");
    let mut task_id: String = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");
    let task_id_number = convert_string_to_number(&task_id);
    tasks.remove(&task_id_number);
    if let Err(err) = save_task_map_to_json_file(&tasks, "tasks.json") {
        eprintln!("Error: {}", err);
    }
}

fn save_task_map_to_json_file(map: &HashMap<u8, Task>, file_path: &str) -> io::Result<()>
where
    Task: Serialize,
{
    let json = serde_json::to_string(map)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() {
    println!("------To Do List!------");

    let mut tasks: HashMap<u8, Task> = HashMap::new();

    if let Err(err) = save_task_map_to_json_file(&tasks, "tasks.json") {
        eprintln!("Error: {}", err);
    }

    loop {
        println!("------Select an option------");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. Show open tasks");
        println!("4. Delete a task");
        println!("5. End");
        println!("----------------------------");

        let mut option_selected = String::new();
        io::stdin()
            .read_line(&mut option_selected)
            .expect("Failed to read line");
        let option_number: u8 = convert_string_to_number(&mut option_selected);

        match option_number {
            1 => add_task(&mut tasks),
            2 => complete_task(&mut tasks),
            3 => show_open_tasks(&tasks),
            4 => delete_task(&mut tasks),
            5 => break,
            _ => println!("Invalid"),
        };
    }
}
