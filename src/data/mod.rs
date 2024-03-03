pub mod download;
pub use download::*;

pub mod edit;
pub use edit::*;

use core::panic;
use std::{
    fs::read_to_string, 
    process::Command
};

pub fn get_lib(lib: String) -> Option<(String, String)> {
    let mut path = get_path();
    path.push_str("libs.txt");
    let file = read_to_string(path).unwrap();

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();
        if parts.get(0).unwrap().trim() == lib {
            return Some((String::from(parts.get(1).unwrap().trim()), String::from(parts.get(2).unwrap().trim())));
        }
    }
    None
}

pub fn get_package_libs(package: String) -> Option<Vec<String>> {
    let mut path = get_path();
    path.push_str("package.txt");
    let file = read_to_string(path).unwrap();

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

        if parts.get(0).unwrap().trim() == package {
            let libs: Vec<String> = parts.get(1).unwrap().trim().split(" ").map(str::to_string).collect();
            return Some(libs);
        }
    }
    None
}

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
