use serde_json::{self, json};
use std::{
    fs::{OpenOptions,read_to_string},
    io::{self, Write},
    process::Command
};

pub struct DataManage {
    json: serde_json::Value,
}

impl DataManage {
    pub fn new() -> DataManage {
        let content: serde_json::Value = serde_json::from_slice(
            read_to_string("libs/libs.json").unwrap().as_bytes())
            .unwrap();

        return DataManage {json: content};
    }

    pub fn download(&self, args: &Vec<String>) {
        for arg in args.iter() {
            for (k, v) in self.json.as_object().unwrap() {
                if k == arg.trim() {
                    for (s, l) in v.as_object().unwrap() {
                        if s == "github" {
                            let output = Command::new("git")
                                .arg("clone")
                                .arg(l.as_str().unwrap())
                                .spawn()
                                .expect("ERROR: DataManage -> download -> Command -> new")
                                .wait()
                                .unwrap();
                            println!("{:?}", output);
                        }
                    }
                }
            }
        }
    }

    pub fn add(&mut self) {
        let mut library_name = String::new();
        let mut git_link = String::new();
       
        print!("Library Name: ");
        io::stdout()
            .flush()
            .expect("ERROR: DataManage -> add -> flush");
        io::stdin()
            .read_line(&mut library_name)
            .expect("ERROR: DataManage -> add -> read_line");

        print!("Git Link: ");
        io::stdout()
            .flush()
            .expect("ERROR: DataManage -> add -> flush");
        io::stdin()
            .read_line(&mut git_link)
            .expect("ERROR: DataManage -> add -> read_line");

        let mut new_library = json!({
            library_name.trim(): {
                "github": git_link.trim()
            }
        });


        let mut json = self.json.clone();

        self.update_with(&mut json, &mut new_library);

        self.update_json(json);
    }

    pub fn remove(&mut self) {
        let mut library_name = String::new();
        print!("Library Name: ");
        io::stdout()
            .flush()
            .expect("ERROR: DataManage -> delete -> flush");
        io::stdin()
            .read_line(&mut library_name)
            .expect("ERROR: DataManage -> delete -> read_line");

        let mut is_sure = false;

        loop {
            let mut is_sure_input = String::new();
            print!("Are You Sure? (yes or no): ");
            io::stdout()
                .flush()
                .expect("ERROR: DataManage -> delete -> flush");
            io::stdin()
                .read_line(&mut is_sure_input)
                .expect("ERROR: DataManage -> delete -> read_line");

            if is_sure_input.trim() == "yes" {
                is_sure = true;
                break;
            } else if is_sure_input.trim() == "no" {
                break;
            }
        }

        if is_sure {
            let mut json = self.json.clone();
            json.as_object_mut()
                .expect("ERROR: DataManage -> delete -> as_object_mut")
                .remove(library_name.as_str().trim());
            self.update_json(json);
        }
    }

    fn update_with(&self, dest: &mut serde_json::Value, src: &serde_json::Value) {
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

    fn update_json(&self, json: serde_json::Value) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("libs/libs.json")
            .expect("ERROR: DataManage -> update_json -> OpenOptions -> open");

        file.set_len(0)
            .expect("ERROR: DataManage -> update_json -> set_len");

        file.write(
            json
            .to_string()
            .as_bytes()
            ).expect("DataManage -> update_json -> write");
    }
}
