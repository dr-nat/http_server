use std::env;
use std::error::Error;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/Request were provided").into());
    }

    let folder_path = &args[1];

    Ok(())
}
