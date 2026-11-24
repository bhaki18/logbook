use std::io;
use crate::libs::file_sys::reader;
use dialoguer::{Select, theme::ColorfulTheme};
use std::env;
use std::process::Command;

pub fn modify_memory()-> Result<u32,io::Error>{
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let memories_dir = logbook_mem_dir.clone() + "/memories";

    let memories_list = reader::read_only_folders(memories_dir.clone())?;
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

    let selected_dir = memories_dir.clone() + "/" + &options[scelta];

    let memories_list_2 = reader::read_only_files(selected_dir.clone())?;
    let mut options_2 = Vec::new(); 

    for os_str in memories_list_2{
        options_2.push(os_str.to_string_lossy().into_owned());
    }

    let scelta_2 = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options_2)
        .items(&["Exit"])
        .default(0)
        .interact()
        .unwrap();
    
    if scelta_2 == options_2.len(){
        return Ok(0);
    }

    let memory_selected_path = selected_dir + "/" + &options_2[scelta_2];

    let output = Command::new("nano")
    .arg(memory_selected_path)
    .status()?;
    

    Ok(0)
}