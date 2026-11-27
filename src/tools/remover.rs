use std::io;
use std::process::Command;

pub fn remove_file_or_folder(path:String) -> Result<u32,io::Error>{
    let output = Command::new("rm")
    .arg("-r")
    .arg(path)
    .output()?;

    Ok(0)
}