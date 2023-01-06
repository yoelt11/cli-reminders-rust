/* 
 The std::env provides functions and types for accessing information
 about the env in which a rust program is running.
 This includes things like command-lind args, env variables and the cwd
*/
use std::env;
use std::error::Error;

// custom mod containing main functions
mod functions;

const USAGE: &str= "Usage: reminder add <message> | list | delete";

fn main() -> Result<(), Box<dyn Error>>{
// main returns a Result where Ok is an empty tupple and Box is a error stored in heap 
    // reads arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // if argument less than 2 return Usage
    if args.len() < 2 {
        println!("{}", USAGE);
        return Ok(()); // breaks code with ok
    }
    
    // get command
    let command =  &args[1];
    

    // match the args with desired functionality
    match command.as_str(){
        "add" => {
            if args.len() < 3 {
                println!("Error: missing message");
                return Ok(());
            }
            
            let message = &args[2..].join(" ");
            let result = functions::add(message);
            
            // check result of function
            if result.is_ok() {
                println!("Reminder added");
            } else {
                println!("Error adding reminder");
            }
        }
        "list" => {

            let result = functions::list();
            
            // check result of function
            if result.is_ok() {
                println!("Listing reminders");
            } else {
                println!("Error displaying reminders");
            }
        }
        "delete" => {

            if args.len() < 3 {
                println!("Error: missing line number");
                return Ok(());
            }

            let line_num: i32 =  args[2].parse().unwrap();
            let result = functions::delete(line_num);
            
            // check result of function
            if result.is_ok() {
                println!("Success deleting reminders");
            } else {
                println!("Error deleting reminders");
            }
        }
        _ => println!{"Error: invalid command"}
    }
    
    Ok(())
}
