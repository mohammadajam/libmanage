use super::get_path;
use std::{
    fs::{
        OpenOptions,
        read_to_string,
        write
    }, 
    io::{
        self, 
        Write
    }, 
    process::exit
};

pub fn add() {
    let mut library_name = String::new();
    let mut library_source = String::new();
    let mut library_link = String::new();

    print!("Library name: ");
    io::stdout()
        .flush()
        .expect("Error: flush");
    io::stdin()
        .read_line(&mut library_name)
        .expect("Error: read_line");

    print!("Library source\n(1) git\n(2) wget\n");
    io::stdout()
        .flush()
        .expect("Error: flush");
    io::stdin()
        .read_line(&mut library_source)
        .expect("Error: read_line");

    print!("Library link: ");
    io::stdout()
        .flush()
        .expect("Error: flush");
    io::stdin()
        .read_line(&mut library_link)
        .expect("Error: read_line");


    let mut path = get_path();
    path.push_str("libs.txt");

    let file = read_to_string(&path).unwrap();

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

        if parts.get(0).unwrap().trim() == library_name.trim() {
            eprintln!("{} Library already exist", library_name.trim());
            exit(0);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let lib_info = format!("{} :: {} :: {}", library_name.trim(), library_source.trim(), library_link.trim());
    if let Err(err) = writeln!(file, "{}", lib_info) {
        eprintln!("Error: Unable to write to file: {}", err);
    }
}

pub fn remove(data_file: &str) {
    let mut name = String::new();
    print!("Name: ");
    io::stdout()
        .flush()
        .expect("ERROR remove -> flush");
    io::stdin()
        .read_line(&mut name)
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
        let mut path = get_path();
        path.push_str(data_file);
        let file = read_to_string(&path).unwrap();

        let mut new_file: String = String::new();

        let mut found_line = false;

        for line in file.lines() {
            let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

            if parts.get(0).unwrap().trim() != name.trim() {
                new_file.push_str(line);
                new_file.push('\n');
            } else { found_line = true; }
        }
        write(path, new_file).expect("Error write to file");
        if !found_line {
            println!("{} Was not found", name.trim());
        }
    }
}

pub fn add_package() {
    let mut package_name = String::new();

    print!("Package Name: ");
    io::stdout()
        .flush()
        .expect("Error flush");
    io::stdin()
        .read_line(&mut package_name)
        .expect("Error read_line");

    let mut package_libs: Vec<String> = Vec::new();

    println!("Type (!) to stop");

    loop {
        let mut input = String::new();

        print!("Library {}: ", package_libs.len()+1);
        io::stdout()
            .flush()
            .expect("Error flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Error read_line");

        if input.trim() == "!" {
            break;
        }
        package_libs.push(String::from(input.trim()));
    }

    package_libs.sort();
    package_libs.dedup();

    let mut path = get_path();
    path.push_str("package.txt");

    let file = read_to_string(&path).unwrap();

    for line in file.lines() {
        let parts: Vec<String> = line.split("::").map(str::to_string).into_iter().collect();

        if parts.get(0).unwrap().trim() == package_name.trim() {
            eprintln!("{} Package already exist", package_name.trim());
            exit(0);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let package_info = format!("{} :: {}", package_name.trim(), package_libs.join(" "));
    if let Err(err) = writeln!(file, "{}", package_info) {
        eprintln!("Error: Unable to write to file: {}", err);
    }
}
