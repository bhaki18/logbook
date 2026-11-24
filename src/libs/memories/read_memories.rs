use crate::libs::file_sys::reader;
use std::io;
use std::env;

pub fn read_memories()-> Result<Vec<String>,io::Error>{
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let memories_dir = logbook_mem_dir.clone() + "/memories";

    let mut ret = Vec::new();

    for os_str in reader::read_only_folders(memories_dir)?{
        ret.push(os_str.to_string_lossy().into_owned());
    }

    Ok(ret)
    
} 