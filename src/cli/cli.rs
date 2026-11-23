use crate::libs::file_sys::start_fs::start_fs;
use super::action_selector::select_action;
use std::io;
use crate::tools::memory_creator::create_new_memory;
use crate::tools::modify_memory::modify_memory;


pub fn cli()-> Result<u32,io::Error>{
    let mut status_code: u32;
    status_code = start_fs()?;
    loop{
    let mut scelta = select_action();
    return match scelta{
        0 => create_new_memory(), //write a new memory
        1 =>  modify_memory(),//modify a memory
        2 =>  Ok(0),
        3 =>  Ok(0),
        4 =>  Ok(0),
        5 =>  Ok(0),
        6 =>  Ok(0),
        7 =>  Ok(0),
        _ =>  Ok(0)
    }    
    }

    Ok(0)
}