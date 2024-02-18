use std::env::*;
use std::fs::File;
use std::io::Read;

fn main() {
    
    // Creating PathBuf object, that contains current directory + path.push(...) 
    let mut file_path = current_dir();
    match &mut file_path {
        Ok(path) => {
            path.push("src/text.txt");
            println!("It readed a file path!");
            println!("\n{:?}", path);
        }, 
        
        Err(err) => {
            eprintln!("Error has occured: {}", err);
        }};
    
    // Converting file_path to String type instead of Result<PathBuf, Error>
    let file_path = match file_path {
        Ok(path_buf) => path_buf.to_string_lossy().to_string(),
        Err(_err) => panic!("PROGRAM PANICKEEEEEED!!!!!")
    };

    // Opening the file, else returning error
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening the file {}", err);
            return;
        }
    };

    // Reading file content 
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => println!("File content: \n{}", content),
        Err(err) => eprintln!("Error has occured: {}", err)
    }
}   