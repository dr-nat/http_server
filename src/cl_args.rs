use std::env;
use std::path::PathBuf;
use std::error::Error;

pub fn get_args() -> Result<PathBuf, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments/Request were provided").into());
    }

    let first_arg = args[1].clone();
    
    // so here we want to handle errors where if a user enters 
    // an argument that is not a directory, then we dont use it, 
    // so we convert the folder to  type of metadata, so we can 
    // use the dir method to check if path is a folder and then handle errors
    // so we store the folder in a vavriable for future  use.

   let folder_path = PathBuf::from(first_arg);  // so here we convert the string gotten from the first argument to a path buf and then get  
                                               // get the metadata to check if it is a directory and then return the path for future use.

   folder_path.metadata()?;

   if folder_path.is_dir() {
        return Ok(folder_path);
   } else {
        Err("No such file or directory".into())
   }

}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_args() -> Result<(), Box<dyn Error>>{
        //let path: Vec<String> = vec![r"/home/natty/Desktop".to_string()];
        let mut path_buf = PathBuf::new();
        path_buf.push(r"/home/natty/Desktop");
        assert_eq!(get_args()?, path_buf);

        Ok(())
    } 


}
