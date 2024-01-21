pub mod install;
pub use install::*;

pub mod list;
pub use list::*;

use std::{
    fs::{read_to_string, File},
    io::{BufReader, BufRead},
    process::Command
};

#[allow(dead_code)]
fn get_json() -> serde_json::Value {
    let path = get_path();
    serde_json::from_slice(
        read_to_string(path)
        .unwrap()
        .as_bytes())
        .unwrap()
}


#[allow(dead_code)]
pub fn get_path() -> String {
    let mut path = String::from_utf8(
        Command::new("bash")
        .arg("-c")
        .arg("echo $LIBMANAGE_DATA")
        .output()
        .unwrap()
        .stdout)
        .unwrap();
    path.pop();
    if path == "" {
        return String::from("data/libs.json");
    }

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
