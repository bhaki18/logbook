use std::io;
use std::process::Command;
use std::env;
use std::fs;
use chrono::{Local, Utc};

pub fn create_new_memory()-> Result<u32,io::Error>{
    let local_now = Local::now();
    let date = local_now.format("%Y-%m-%d");
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let mut memory_path = logbook_mem_dir + "/" + "memories/" + &date.to_string();
    let mut memory_name = String::new();

    println!("what name do you wanna give to your memory?");
    io::stdin().read_line(&mut memory_name);
    memory_path = memory_path + "/" + &memory_name;
    
    let output = Command::new("nano")
    .arg(memory_path)
    .status()?;

    Ok(0)
}