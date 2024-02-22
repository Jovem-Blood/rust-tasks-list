use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::methods::FILE_PATH;

pub struct Task {
    title: String,
    description: String,
}

impl Task {
    pub fn new(title: String, description: String) -> Task {
        Task { title, description }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn to_string(&self) -> String {
        format!("{};{}\n", self.get_title(), self.get_description())
    }

    pub fn print(&self, index: usize) {
        println!("ID: {}", index);
        println!("Title: {}", self.get_title());
        println!("Description: {}", self.get_description());
        println!("--------------------------");
    }
}

pub fn read_tasks_file() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(FILE_PATH)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let tasks = content
        .lines()
        .map(|line| line.split(";").map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|parts| Task::new(parts[0].clone(), parts[1].clone()))
        .collect();
    Ok(tasks)
}

pub fn write_tasks_file(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let file = File::create(FILE_PATH)?;
    let mut buffer = BufWriter::new(file);

    let mut buffer_string = String::new();

    for task in tasks.as_slice() {
        buffer_string.push_str(&task.to_string())
    }

    buffer.write_all(buffer_string.as_bytes())?;
    buffer.flush()?;

    Ok(())
}
