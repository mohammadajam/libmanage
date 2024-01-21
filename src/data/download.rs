use super::get_json;
use std::process::Command;


pub fn download(libs: Vec<String>) {
    let json  = get_json();

    for arg in libs.iter() {
        for (lib, value) in json.as_object().unwrap() {
            if lib == arg.trim() {
                for (content, link) in value.as_object().unwrap() {
                    if content == "github" {
                        let output = Command::new("git")
                            .arg("clone")
                            .arg(link.as_str().unwrap())
                            .spawn()
                            .expect("Error Command")
                            .wait()
                            .unwrap();
                        println!("{output:?}");
                    }
                }
            }
        }
    }

}
