use crate::libs::file_sys::start_fs::start_fs;
use super::action_selector::select_action;
use std::io;

pub fn cli()-> Result<u32,io::Error>{
    let mut status_code: u32;
    status_code = start_fs()?;
    loop{
    let mut scelta = select_action();
    match scelta{
        0 => println!(""), //write a new memory
        1 => println!(""),
        2 => println!(""),
        3 => println!(""),
        4 => println!(""),
        5 => println!(""),
        6 => println!(""),
        7 => println!(""),
        _ => println!("")
    }
    }
}