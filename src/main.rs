use std::env;

mod arg_manage;
mod data_manage;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut arg = arg_manage::ArgManage::new(args);

    arg.excute();

}
