use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
};

use clearscreen::clear;

fn prompt(text: &str, option: &mut String) {
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(option)
        .expect("Error while reading line");
    *option = option.trim().to_string()
}

fn options_menu() {
    println!("[C] create new task");
    println!("[R] read a task");
    println!("[U] update a task");
    println!("[D] delete a task");
    println!("[Q] quit");
}

fn create_task() {
    let mut title = String::new();
    let mut description = String::new();
    prompt("What is the task's title? ", &mut title);
    prompt("Give a description to it: ", &mut description);

    let mut data_file = OpenOptions::new()
        .append(true)
        .open("./tasks.txt")
        .expect("Could not open this file :/");

    let new_content = format!("{};{}\n", title, description);
    data_file.write(new_content.as_bytes()).unwrap();
}

fn delete_task() {
    clear().unwrap();
    let file_content = fs::read_to_string("./tasks.txt").expect("Could not open this file");

    for (index, line) in file_content.lines().enumerate() {
        let split: Vec<&str> = line.split(";").collect();
        println!("ID: {}", index);
        println!("Title: {}", split[0]);
        println!("Description: {}", split[1]);
        println!("--------------------------");
    }

    let mut lines: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();
    let mut ind = String::new();
    prompt("Which one you want to delete? ", &mut ind);
    let index = ind.parse::<usize>().unwrap();
    lines.remove(index);
    let full_content = lines.join("\n");
    let mut f = File::create("./tasks.txt").unwrap();
    f.write_all(full_content.as_bytes()).unwrap();
}

fn update_tasks() {
    clear().unwrap();
    let file_content = fs::read_to_string("./tasks.txt").expect("Could not open this file");

    for (index, line) in file_content.lines().enumerate() {
        let split: Vec<&str> = line.split(";").collect();
        println!("ID: {}", index);
        println!("Title: {}", split[0]);
        println!("Description: {}", split[1]);
        println!("--------------------------");
    }

    let mut lines: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();
    let mut ind = String::new();
    prompt("Which one you want to update? ", &mut ind);
    let index = ind.parse::<usize>().unwrap();
    let editing_line = lines[index].clone();
    let mut opt = String::new();
    prompt(
        "Type 1 for edit the title and 2 for the description",
        &mut opt,
    );
    let mut fields: Vec<&str> = editing_line.split(";").collect();
    let mut new_content = String::new();
    match opt.as_str() {
        "1" => {
            prompt("New title: ", &mut new_content);
            fields[0] = new_content.as_str();
        }
        "2" => {
            prompt("New descriptuon: ", &mut new_content);
            fields[1] = new_content.as_str();
        }
        _ => {
            println!("This value is ilegal")
        }
    }
    lines[index] = fields.join(";");
    let full_content = lines.join("\n");
    let mut f = File::create("./tasks.txt").unwrap();
    f.write_all(full_content.as_bytes()).unwrap();
}

fn read_tasks() {
    clear().unwrap();
    match fs::read_to_string("./tasks.txt") {
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

fn main() {
    let mut option = String::new();

    loop {
        option.clear();
        options_menu();
        prompt("Enter your option: ", &mut option);
        option = option.to_uppercase();

        match option.as_str() {
            "C" => {
                create_task();
            }
            "R" => {
                read_tasks();
            }
            "U" => {
                update_tasks();
            }
            "D" => {
                delete_task();
                println!("Deleting task")
            }
            "Q" => {
                println!("Quiting");
                break;
            }
            f => {
                println!("The option {} is invalid", f);
            }
        }
        println!("")
    }
}
