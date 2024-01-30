use super::update_data;
use super::update_with;

use super::get_json;
use std::io::{self, Write};
use serde_json::json;

pub fn add() {
    let mut json: serde_json::Value = get_json();

    let mut library_name = String::new();
    let mut git_link = String::new();

    print!("Library Name: ");
    io::stdout()
        .flush()
        .expect("ERROR add -> flush");
    io::stdin()
        .read_line(&mut library_name)
        .expect("ERROR add -> read_line");

    print!("Git Link: ");
    io::stdout()
        .flush()
        .expect("ERROR add -> flush");
    io::stdin()
        .read_line(&mut git_link)
        .expect("ERROR add -> read_line");

    let new_library = json!({
        library_name.trim(): {
            "github": git_link.trim()
        }
    });



    update_with(&mut json, &new_library);
    update_data(json);
}


pub fn remove() {
    let json = get_json();
    let mut library_name = String::new();
    print!("Library Name: ");
    io::stdout()
        .flush()
        .expect("ERROR remove -> flush");
    io::stdin()
        .read_line(&mut library_name)
        .expect("ERROR remove -> read_line");

        let mut is_sure = false;

        loop {
            let mut is_sure_input = String::new();
            print!("Are You Sure? (yes or no): ");
            io::stdout()
                .flush()
                .expect("ERROR remove -> flush");
            io::stdin()
                .read_line(&mut is_sure_input)
                .expect("ERROR remove -> read_line");

            if is_sure_input.trim() == "yes" {
                is_sure = true;
                break;
            } else if is_sure_input.trim() == "no" {
                break;
            }
        }

        if is_sure {
            let mut json = json.clone();
            json.as_object_mut()
                .expect("ERROR remove -> as_object_mut")
                .remove(library_name.as_str().trim());
            update_data(json);
        }
    }
