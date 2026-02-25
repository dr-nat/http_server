use std::env;
use std::error::Error;
use std::fs::metadata;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/Request were provided").into());
    }

    let folder_path = args[1].clone();
    
    // so here we want to handle errors where if a user enters 
    // an argument that is not a directory, then we dont use it, 
    // so we convert the folder to  type of metadata, so we can 
    // use the dir method to check if path is a folder and then handle errors
    // so we store the folder in a vavriable for future  use.
    
    let dir = metadata(folder_path)?;

    let _my_folder = if dir.is_dir() {
        let _work_space = dir;
    } else {
        return Err(format!("No such file or directory").into());
    };

    Ok(())
}
