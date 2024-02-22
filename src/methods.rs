use std::{
    error::Error,
    io::{self, Write},
};

use crate::task::*;

pub const FILE_PATH: &str = "./tasks.txt";

pub fn prompt(text: &str) -> Result<String, Box<dyn Error>> {
    let mut option = String::new();
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut option)
        .map(|_| option.trim().to_string())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn options_menu() {
    println!("[C] create new task");
    println!("[R] read a task");
    println!("[U] update a task");
    println!("[D] delete a task");
    println!("[Q] quit");
}

pub fn create_task() -> Result<(), Box<dyn Error>> {
    let title = prompt("What is the task's title? ")?;
    let description = prompt("Give a description to it: ")?;
    let task = Task::new(title, description);
    let mut tasks = read_tasks_file().unwrap();
    tasks.push(task);
    write_tasks_file(&tasks)
}

pub fn delete_task() -> Result<(), Box<dyn Error>> {
    let tasks = &mut read_tasks_file().unwrap();

    for (index, task) in tasks.into_iter().enumerate() {
        task.print(index)
    }

    loop {
        let ind = prompt("Which one you want to delete? ").unwrap();
        if let Ok(index) = ind.parse::<usize>() {
            if index < tasks.len() {
                tasks.remove(index);
                return write_tasks_file(tasks);
            } else {
                println!(
                    "Invalid index. Please enter a number between 0 and {}",
                    tasks.len() - 1
                );
                continue;
            }
        } else {
            println!("Please, Enter a valid number");
            continue;
        }
    }
}

pub fn update_tasks() -> Result<(), Box<dyn Error>> {
    let tasks = &mut read_tasks_file().unwrap();

    for (index, task) in tasks.into_iter().enumerate() {
        task.print(index)
    }

    loop {
        let ind = prompt("Which one you want to update? ").unwrap();
        if let Ok(index) = ind.parse::<usize>() {
            if index < tasks.len() {
                let task = tasks.get_mut(index).unwrap();
                let opt = prompt("Type 1 for edit the title and 2 for the description ").unwrap();

                match opt.as_str() {
                    "1" => task.set_title(prompt("New title: ").unwrap()),
                    "2" => task.set_description(prompt("New description: ").unwrap()),
                    _ => println!("This value is ilegal"),
                }

                break;
            } else {
                println!(
                    "Invalid index. Please enter a number between 0 and {}",
                    tasks.len() - 1
                );
                continue;
            }
        } else {
            println!("Please, Enter a valid number");
            continue;
        }
    }
    write_tasks_file(tasks)
}

pub fn read_tasks() -> Result<(), Box<dyn Error>> {
    read_tasks_file()
        .map(|tasks| {
            for (index, task) in tasks.into_iter().enumerate() {
                task.print(index)
            }
        })
        .map_err(|e| e)
}
