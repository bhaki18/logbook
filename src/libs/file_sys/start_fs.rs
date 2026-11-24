use std::env;
use std::fs;
use owo_colors::OwoColorize;
use std::io;

pub fn start_fs() -> io::Result<u32>{// return status code
    let home = env::var("HOME").unwrap(); 
    let logbook_mem_dir = home.clone() + "/.logbook_memories";
    let memories_dir = logbook_mem_dir.clone() + "/memories";
    let works_dir = logbook_mem_dir.clone() + "/works";

    let logbook_mem_dir_output = check_path(logbook_mem_dir.clone())?;
    if !logbook_mem_dir_output{
        let _ = create_path(logbook_mem_dir.clone())?;
    }

    let memories_dir_output = check_path(memories_dir.clone())?;
    if !memories_dir_output{
        let _ = create_path(memories_dir.clone())?;
    }

    let works_dir_output = check_path(works_dir.clone())?;
    if !works_dir_output{
        let _ = create_path(works_dir)?;
    }

    return Ok(0);
}

pub fn check_path(path:String)-> io::Result<bool>{
    let ret = fs::exists(&path)?;

    Ok(ret)

}   

pub fn create_path(path:String) -> io::Result<u32>{
    let ret = fs::create_dir_all(&path);

    if ret.as_ref().is_ok(){
        return Ok(0);
    }else if ret.is_err(){
        println!("{:?}",("FATAL ERROR:Unable to create path:{}",path));
        return Err(io::Error::new(io::ErrorKind::PermissionDenied,"7"));
    }else{
        return Err(io::Error::new(io::ErrorKind::Other,"8"));
    }
}