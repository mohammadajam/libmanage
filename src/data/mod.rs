pub mod download;
pub use download::*;

pub mod edit;
pub use edit::*;

use core::panic;
use std::{
    fs::{read_to_string, OpenOptions},
    io::Write, 
    process::Command
};

fn get_json() -> serde_json::Value {
    let mut path = get_path();

    path.push_str("libs.json");
    let path = String::from(path);

    let text = read_to_string(path);

    match text {
        Ok(content) => {
            let json: serde_json::Value  = serde_json::from_slice(content.as_bytes()).unwrap();
            return json;
        }

        Err(err) => panic!("ERROR READ FILE {err:?}")
    }
}

fn get_package_json() -> serde_json::Value {
    let mut path = get_path();

    path.push_str("library_package.json");
    path = String::from(path);

    let text = read_to_string(path);

    match text {
        Ok(content) => {
            let json: serde_json::Value = serde_json::from_slice(content.as_bytes()).unwrap();
            return json;
        }
        Err(err) => panic!("ERROR READ library_packages ERR {err:?}")
    }
}

fn update_data(json: serde_json::Value) {
    let json_path = get_path();
    let mut file = OpenOptions::new()
        .append(true)
        .open(json_path.clone())
        .expect("ERROR: DataManage -> update_json -> OpenOptions -> open");

    file.set_len(0)
        .expect("ERROR: DataManage -> update_json -> set_len");

    file.write(
        json
        .to_string()
        .as_bytes()
        ).expect("ERROR update_data WRITE ERROR");
}

pub fn update_with(dest: &mut serde_json::Value, src: &serde_json::Value) {
        use serde_json::Value::{Object, Null};
    
        match (dest, src) {
            (
                &mut Object(ref mut map_dest),
                &Object(ref map_src),
            ) => {
            // map_dest and map_src both are Map<String, Value>
                for (key, value) in map_src {
                // if key is not in map_dest, create a Null object
                // then only, update the value
                    *map_dest
                        .entry(key.clone())
                        .or_insert(Null) = value.clone();
                }
            }
            (_, _) => panic!("update_with only works with two serde_json::Value::Object s"),
        }
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
    

    if path == "" {
        return String::from("data/");
    }

    path.pop();

    path
}
