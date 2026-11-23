use std::fs;
use std::io;
use std::ffi;

pub fn read_only_folders(path:String)-> Result<Vec<ffi::OsString>,io::Error>{
    let content = fs::read_dir(path)?;
    let mut ret = Vec::new();

    for i in content{
        let cont = &i?;
        let metadata = cont.file_type()?;
        if metadata.is_dir(){
            let to_push = cont.file_name();
            ret.push(to_push);
        }
    }

    Ok(ret)
}

pub fn read_only_files(path:String)-> Result<Vec<ffi::OsString>,io::Error>{
    let content = fs::read_dir(path)?;
    let mut ret = Vec::new();

    for i in content{
        let cont = &i?;
        let metadata = cont.file_type()?;
        if metadata.is_file(){
            let to_push = cont.file_name();
            ret.push(to_push);
        }
    }

    Ok(ret)
}


pub fn read_all(path:String)-> Result<Vec<ffi::OsString>,io::Error>{
    let content = fs::read_dir(path)?;
    let mut ret = Vec::new();

    for i in content{
        let cont = &i?;
        let to_push = cont.file_name();
        ret.push(to_push);
    }

    Ok(ret)
}