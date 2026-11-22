use std::env;
use std::fs;
use owo_colors::OwoColorize;

pub fn start_fs() -> u32{// return status code
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";

    let check_path = fs::exists(&logbook_mem_dir);

    if check_path.as_ref().is_ok_and(|x| *x == false){
        println!("logbook dir not found...");
        println!("creating path:{}",logbook_mem_dir);
        if fs::create_dir_all(&logbook_mem_dir).is_ok(){
            println!("path created successful:{}",logbook_mem_dir);
        }else{
            println!("FATAL ERROR:failed creating path:{}",logbook_mem_dir);
            return 1;
        }
    }

    if check_path.is_err(){
        println!("{:?}",("FATAL ERROR:Unable to check logbook path:{}".red(),logbook_mem_dir.clone()));
        return 2;
    }

    if fs::exists(logbook_mem_dir).is_ok_and(|x| x == true){
        return 0;
    }

    return 3;
}