use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, Write},
};

use clearscreen::clear;

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

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(FILE_PATH)
        .expect("Could not open this file :/");

    let new_content = format!("{};{}\n", title, description);
    data_file.write(new_content.as_bytes())?;
    Ok(())
}

pub fn delete_task() -> Result<(), Box<dyn Error>> {
    clear().unwrap();
    let file_content = fs::read_to_string(FILE_PATH).expect("Could not open this file");

    for (index, line) in file_content.lines().enumerate() {
        let split: Vec<&str> = line.split(";").collect();
        println!("ID: {}", index);
        println!("Title: {}", split[0]);
        println!("Description: {}", split[1]);
        println!("--------------------------");
    }

    loop {
        let mut lines: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();
        let ind = prompt("Which one you want to delete? ").unwrap();
        if let Ok(index) = ind.parse::<usize>() {
            if index < lines.len() {
                lines.remove(index);
                let full_content = lines.join("\n");
                let mut f = File::create(FILE_PATH).unwrap();
                f.write_all(full_content.as_bytes())?;
                return Ok(())
            } else {
                println!("Invalid index. Please enter a number between 0 and {}", lines.len() - 1);
                continue;
            }
        } else {
            println!("Please, Enter a valid number");
            continue;
        }

    }
}

pub fn update_tasks() {
    clear().unwrap();
    let file_content = fs::read_to_string(FILE_PATH).expect("Could not open this file");

    for (index, line) in file_content.lines().enumerate() {
        let split: Vec<&str> = line.split(";").collect();
        println!("ID: {}", index);
        println!("Title: {}", split[0]);
        println!("Description: {}", split[1]);
        println!("--------------------------");
    }

    let mut lines: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();
    let ind = prompt("Which one you want to update? ").unwrap();
    let index = ind.parse::<usize>().unwrap();
    let editing_line = lines[index].clone();
    let opt = prompt("Type 1 for edit the title and 2 for the description ").unwrap();
    let mut fields: Vec<String> = editing_line.split(";").map(|s| s.to_string()).collect();

    match opt.as_str() {
        "1" => {
            let new_value = prompt("New title: ").unwrap();
            fields[0] = new_value;
        }
        "2" => {
            let new_value = prompt("New description: ").unwrap();
            fields[1] = new_value;
        }
        _ => {
            println!("This value is ilegal")
        }
    }

    lines[index] = fields.join(";");
    let full_content = lines.join("\n");
    let mut f = File::create(FILE_PATH).unwrap();
    f.write_all(full_content.as_bytes()).unwrap();
}

pub fn read_tasks() {
    clear().unwrap();
    match fs::read_to_string(FILE_PATH) {
        Ok(file_content) => {
            for (index, line) in file_content.lines().enumerate() {
                let split: Vec<&str> = line.split(";").collect();
                println!("ID: {}", index);
                println!("Title: {}", split[0]);
                println!("Description: {}", split[1]);
                println!("--------------------------");
            }
        }
        Err(_) => {
            println!("Could not open this file");
        }
    }
}

