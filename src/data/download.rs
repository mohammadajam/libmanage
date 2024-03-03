use super::{get_lib, get_package_libs};
use std::process::Command;

pub fn download(libs: &Vec<String>) {
    for lib in libs.iter() {
        if let Some((source, link)) = get_lib(String::from(lib)) {
            if source == "git" {
                let output = Command::new(&source)
                    .arg("clone")
                    .arg(&link)
                    .spawn()
                    .expect("Error git clone")
                    .wait()
                    .unwrap();
                println!("{output}\n");
            }
            if source == "wget" {
                let output = Command::new(&source)
                    .arg(&link)
                    .spawn()
                    .expect("Error wget")
                    .wait()
                    .unwrap();
                println!("{output}\n");
            }
        }
    }
}

pub fn download_package(packages: &Vec<String>) {
    packages.iter().for_each(|package| {
        if let Some(libs) = get_package_libs(String::from(package)) {
            download(&libs);
        }
    });

}
