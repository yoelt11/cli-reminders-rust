use std::error::Error;
// handles directories paths 
use std::io::{Write};
use std::path::Path;
use std::fs::{self, OpenOptions};

pub fn add(message:&String) -> Result<(), Box<dyn Error>> {
    // create file if not exits
    let reminder_file = Path::new("reminders.txt");
    //let mut file = File::create(reminder_file)?; (always creates a new file:)
    let mut file = OpenOptions::new() // to open file in append mode
        .write(true)
        .append(true)
        .open(reminder_file)
        .unwrap();
    writeln!(file, "{}", message)?;

    Ok(())
}
pub fn delete(line_num:i32) -> Result<(), Box<dyn Error>> {
    // 1) open file in truncate mode
    let reminder_file = Path::new("reminders.txt");
    let contents = fs::read_to_string(reminder_file)?;
    // truncate method deletes all contents from file
    let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(reminder_file)
                    .unwrap();
    
    // 2)filter line to delete rewrite to file
    for (i, line) in contents.lines().enumerate(){

        if i != line_num.try_into().unwrap() {
            println!("[{}] {}", i, line);
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}
pub fn list() -> Result<(), Box<dyn Error>> {
    let reminder_file = Path::new("reminders.txt");
    let contents = fs::read_to_string(reminder_file)?;
    
    // println!("{}", contents);
    // to print the line numbers of each line
    for (i,line) in contents.lines().enumerate(){
        println!("[{}] {}", i, line);
    }

    Ok(())
}