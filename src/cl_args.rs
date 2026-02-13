use std::env;
use std::error::Error;
use std::fs::metadata;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/Request were provided").into());
    }

    let folder_path = args[1].clone();
    
    let dir = metadata(folder_path)?;

    let my_folder = if dir.is_dir() {
        let work_space = dir;
    } else {
        return Err(format!("No such file or directory").into());
    };

    Ok(())
}
