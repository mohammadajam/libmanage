pub mod install;
pub use install::*;

pub mod list;
pub use list::*;

use std::{
    fs::File,
    io::{BufReader, BufRead},
    process::Command
};

pub fn get_path() -> String {
    let cmd_result = Command::new("bash")
        .arg("-c")
        .arg("echo $LIBMANAGE_DATA")
        .output();

    let mut path = match cmd_result {
        Ok(result) => {
            match String::from_utf8(result.stdout) {
                Ok(path) => path,
                Err(err) => panic!("ERROR UNWRAP STRING {err:?}")
            }
        }
        Err(state) => panic!("ERROR ECHO PATH {state:?}")
    };
    

    if path.len() == 1 {
        return String::from("data/");
    }
    path.pop();
    path
}

pub fn line_exist(line: &str, file: &mut File) -> bool {
    let reader = BufReader::new(file);
    
    for file_line in reader.lines() {
        let file_line = file_line.unwrap();
        if file_line.contains(line) {
            return true;
        }
    }
    return false;
}
