mod methods;
mod task;
use std::error::Error;

use clearscreen::clear;
use methods::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut option = String::new();

    loop {
        option.clear();
        options_menu();
        option = prompt("Enter your option: ").unwrap();
        option = option.to_uppercase();

        clear().unwrap();
        match option.as_str() {
            "C" => create_task()?,
            "R" => read_tasks()?,
            "U" => update_tasks()?,
            "D" => delete_task()?,
            "Q" => {
                println!("Quiting");
                break;
            }
            f => {
                println!("The option {} is invalid", f);
            }
        }
    }
    Ok(())
}
