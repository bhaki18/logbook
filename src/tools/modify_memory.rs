use std::io;
use crate::libs::file_sys::reader;
use dialoguer::{Select, theme::ColorfulTheme};
use std::env;

pub fn modify_memory()-> Result<u32,io::Error>{
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let memories_dir = logbook_mem_dir.clone() + "/memories";

    let memories_list = reader::read_only_folders(memories_dir)?;
    let mut options = Vec::new(); 
    for os_str in memories_list{
        options.push(os_str.to_string_lossy().into_owned());
    }

    let scelta = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .items(&["Exit"])
        .default(0)
        .interact()
        .unwrap();
    
    if scelta == options.len(){
        return Ok(0);
    }

    println!("{}",options[scelta]);

    Ok(0)
}