use dialoguer::{Select, theme::ColorfulTheme};
use std::io;
use std::process::Command;
use crate::libs::memories::read_memories;
use crate::libs::works::read_works;
use std::env;

pub fn add_a_file_or_a_folder(copy_path:String,dest_path:String)-> Result<u32,io::Error>{
    let output = Command::new("cp")
    .arg("-r")
    .arg(&copy_path)
    .arg(&dest_path)
    .output()?;

     if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(format!(
            "cp fallito: {}",
            stderr.trim()
        )));
    }
    Ok(0)
}

pub fn select_work() -> Result<u32,io::Error>{
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let works_dir = logbook_mem_dir.clone() + "/works/";


    let options = read_works::read_works()?;

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a work")
        .items(&options)
        .items(&["Exit"])
        .default(0)
        .interact()
        .unwrap();

    if scelta == options.len(){
        return Ok(0);
    }

    let dest = works_dir + &options[scelta];

    
    let mut copy = String::new();

    // read a line into buffer
    println!("what do you wanna add here?(use a relative or absolute path)");
    io::stdin().read_line(&mut copy)?;
    copy = copy.trim().to_string();
    
    if copy.starts_with("~"){
        copy = copy.strip_prefix("~").expect("REASON").to_string();
        copy = home + &copy;
    }


    add_a_file_or_a_folder(copy,dest)?;

    Ok(0)
}


pub fn select_memory()-> Result<u32,io::Error>{
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let memories_dir = logbook_mem_dir.clone() + "/memories/";


    let options = read_memories::read_memories()?;

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a memory")
        .items(&options)
        .items(&["Exit"])
        .default(0)
        .interact()
        .unwrap();

    if scelta == options.len(){
        return Ok(0);
    }

    let dest = memories_dir + &options[scelta];

    
    let mut copy = String::new();

    // read a line into buffer
    println!("what do you wanna add here?(use a relative or absolute path)");
    io::stdin().read_line(&mut copy)?;
    copy = copy.trim().to_string();
    
    if copy.starts_with("~"){
        copy = copy.strip_prefix("~").expect("REASON").to_string();
        copy = home + &copy;
    }

    add_a_file_or_a_folder(copy,dest)?;

    Ok(0)
    
}
