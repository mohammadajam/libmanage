use super::{get_json, get_package_json};
use std::process::Command;


pub fn download(libs: &Vec<String>) {
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

pub fn download_package(packages: &Vec<String>) {
    let package_json = get_package_json();
    let json = get_json();


    for package_name in packages.iter() {
        for (package, list) in package_json.as_object().unwrap() {
            if package_name == package {
                for l in list.as_array().unwrap().iter() {
                    for (lib_name, content) in json.as_object().unwrap() {
                        if l == lib_name {
                            for (source, link) in content.as_object().unwrap() {
                                if source == "github" {
                                    let output = Command::new("git")
                                        .arg("clone")
                                        .arg(link.as_str().unwrap())
                                        .spawn()
                                        .expect("Error git clone")
                                        .wait();

                                    match output {
                                        Ok(status) => {println!("download completed {}", status)},
                                        Err(status) => {println!("download failes {}", status)}
                                        
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


}
