use std::fs::read_to_string;
use super::get_path;

pub fn list() {
    let mut path = get_path();
    path.push_str("libs.txt");
    let file = read_to_string(path).unwrap();

    println!("Library Name       Source             Link\n");

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

        let library_name = parts.get(0).unwrap().trim();
        let library_source = parts.get(1).unwrap().trim();
        let library_link = parts.get(2).unwrap().trim();

        print!("{}", library_name);
        for _ in 1..(20-library_name.len()) {
            print!(" ");
        }

        print!("{}", library_source);
        for _ in 1..(20-library_source.len()) {
            print!(" ");
        }

        println!("{}\n", library_link);
    }
}

pub fn list_package() {
    let mut path = get_path();
    path.push_str("package.txt");
    let file = read_to_string(path).unwrap();

    println!("Package Name       Libraries\n");

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

        let package_name = parts.get(0).unwrap().trim();
        let package_libraries = parts.get(1).unwrap().trim();

        print!("{}", package_name);
        for _ in 1..(20-package_name.len()) {
            print!(" ");
        }

        println!("{}\n", package_libraries);
    }
}
