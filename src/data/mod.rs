pub mod download;
pub use download::*;

pub mod edit;
pub use edit::*;

use std::{
    fs::{read_to_string, OpenOptions},
    io::Write
};
use dirs::home_dir;

fn get_json() -> serde_json::Value {
    let path = home_dir().unwrap().join("Projects/libmanage/libs/libs.json");

    serde_json::from_slice(
        read_to_string(path)
        .unwrap()
        .as_bytes())
        .unwrap()
}

fn update_data(json: serde_json::Value) {
    let json_path = home_dir().unwrap().join("Projects/libmanage/libs/libs.json");
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
        ).expect("DataManage -> update_json -> write");
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
