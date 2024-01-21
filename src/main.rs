use clap::Parser;
mod data;
mod util;

#[derive(Parser)]
struct Cli {
    libs: Vec<String>,

    #[arg(long, action = clap::ArgAction::Count)]
    download_pack: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    download: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    add: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    remove: u8,

    #[arg{long, action = clap::ArgAction::Count}]
    install: u8,
}

fn main() {
    let cli = Cli::parse();
    //util::install();

    match cli.download {
        0 => {},
        1 => {data::download(cli.libs)},
        _ => panic!("ERROR: too many args")
    }

    match cli.add {
        0 => {},
        1 => {data::add()},
        _ => panic!("ERROR: too many args")
    }

    match cli.remove {
        0 => {},
        1 => {data::remove()},
        _ => panic!("ERROR: too many args")
    }

    match cli.install {
        0 => {},
        1 => {util::install()},
        _ => panic!("ERROR: too many args")
    }
}
