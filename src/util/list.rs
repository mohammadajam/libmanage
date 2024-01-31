use super::{get_json, get_package_json};

pub fn list() {
    let json = get_json();

    for (lib, content) in json.as_object().unwrap() {
        println!("{}", lib);
        for (source, link) in content.as_object().unwrap() {
            println!("source: {}     link: {}\n", source, link);
        }
    }
}

pub fn list_package() {
    let package_json = get_package_json();

    for (package, content) in package_json.as_object().unwrap() {
        println!("{package}");
        print!("Libs: ");

        for lib in content.as_array().unwrap().iter() {
            print!("{lib} ");
        }
        println!("\n");
    }
}
