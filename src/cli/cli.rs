use crate::libs::file_sys::start_fs::start_fs;
use super::action_selector::select_action;
use std::io;
use crate::tools::memory_creator::create_new_memory;
use crate::tools::modify_memory::modify_memory;
use crate::tools::file_folder_adder::select_memory;


pub fn cli()-> Result<u32,io::Error>{
    let mut status_code: u32;
    status_code = start_fs()?;
    loop{
    let mut scelta = select_action();
    return match scelta{
        0 => create_new_memory(), //write a new memory
        1 =>  modify_memory(),//modify a memory
        2 =>  select_memory(),// add a file or a folder to your memories
        3 =>  Ok(0),// remove a file or a folder from your memories
        4 =>  Ok(0),// remove one of your memories
        5 =>  Ok(0),
        6 =>  Ok(0),
        7 =>  Ok(0),
        8 =>  Ok(0),
        9 =>  Ok(0),
        10 =>  Ok(0),
        11 =>  Ok(0),
        12 =>  Ok(0),
        13 =>  Ok(0),
        14 =>  Ok(0),
        _ =>  Ok(0)
    }    
    }

    Ok(0)
}