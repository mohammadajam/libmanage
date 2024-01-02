use crate::data_manage::DataManage;
use std::process::exit;

#[derive(Debug)]
enum Mode {
    NONE,
    DOWNLOAD,
    ADD,
    REMOVE
}

pub struct ArgManage {
    args: Vec<String>,
    mode: Mode,
    data_manage: DataManage
}

impl ArgManage {
    pub fn new(args: Vec<String>) -> ArgManage {
        let mut mode = Mode::NONE;

        if args.len() > 1 {
            match args[1].as_str() {
                "-d" => { mode = Mode::DOWNLOAD; },
                "-a" => { mode = Mode::ADD; },
                "-r" => { mode = Mode::REMOVE; },
                _ => {}

            }
        }
        
        return ArgManage {args, mode, data_manage: DataManage::new()};
    }


    pub fn excute(&mut self) {
        match self.mode {
            Mode::NONE => {
                println!("ERROR: NO MODE ENTERED");
                exit(1);
            },
            Mode::DOWNLOAD => {
                self.data_manage.download(&self.args);
            },
            Mode::ADD => {
                self.data_manage.add();
            },
            Mode::REMOVE => {
                self.data_manage.remove();
            }
        }
    }
}
