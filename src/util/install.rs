use std::{
    fs::OpenOptions,
    process::Command,
    io::Write,
};
use dirs::home_dir;

use super::line_exist;


pub fn install() {
    let mut file  = OpenOptions::new()
        .append(true)
        .read(true)
        .open(home_dir().unwrap().join(".bashrc"))
        .expect("Error: open bashrc file");

    let libmanage_dir = home_dir().unwrap().join(".libmanage/");
    
    if !libmanage_dir.exists() {
        Command::new("mkdir")
            .arg(&libmanage_dir)
            .spawn()
            .expect("Error: mkdir .libmanage");
    }

    if !libmanage_dir.join("data/").exists() {
        Command::new("mkdir")
            .arg(&libmanage_dir.join("data/"))
            .spawn()
            .expect("Error: mkdir .libmanage/data");

        Command::new("cp")
            .arg("-n")
            .arg("-a")
            .arg("target/debug/libmanage")
            .arg(&libmanage_dir)
            .spawn()
            .expect("Error: cp .libmanage/libmanage");
    }
    if libmanage_dir.join("data").exists() {
        Command::new("cp")
            .arg("-a")
            .arg("-n")
            .arg("data/.")
            .arg(&libmanage_dir.join("data/"))
            .spawn()
            .expect("Error: cp .libmanage/data");

        Command::new("cp")
            .arg("-n")
            .arg("-a")
            .arg("target/debug/libmanage")
            .arg(&libmanage_dir)
            .spawn()
            .expect("Error: cp .libmanage/libmanage");
    }

    if libmanage_dir.join("libmanage").exists() {
        Command::new("rm")
            .arg(&libmanage_dir.join("libmanage"))
            .spawn()
            .expect("S");

        Command::new("cp")
            .arg("target/debug/libmanage")
            .arg(&libmanage_dir)
            .spawn()
            .expect("DF");
    }

    let libmanage_alias = "alias libmanage=\"$HOME/.libmanage/libmanage\"";
    let libmanage_data = "export LIBMANAGE_DATA=\"$HOME/.libmanage/data/\"";



    if !line_exist(libmanage_alias, &mut file) {
        file.write(libmanage_alias.as_bytes()).expect("Error: write libmanage alias to .bashrc");
        file.write("\n\n".as_bytes()).expect("Error: write \\n to .bashrc");
    }


    file.write(libmanage_data.as_bytes()).expect("Error: write libmanage date export to .bashrc");
    file.write("\n".as_bytes()).expect("Error: write \\n to .bashrc");
}

#[allow(dead_code)]
pub fn uninstall() {
    unimplemented!()
}
