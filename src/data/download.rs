use super::get_json;
use std::{
    env,
    process::Command
};


pub fn download() {
    let args: Vec<String> = env::args().collect();
    let json  = get_json();

    for arg in args.iter() {
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


/*

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


*/
