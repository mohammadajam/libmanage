use clap::Parser;
mod data;
mod util;

#[derive(Parser)]
struct Cli {
    libs: Vec<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    download: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    download_package: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    add: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    remove: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    list: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    list_package: u8,

    #[arg{long, action = clap::ArgAction::Count}]
    install: u8,

    #[arg(long, action = clap::ArgAction::Count)]
    uninstall: u8
}

fn main() {
    let cli = Cli::parse();
    
    match cli.download {
        0 => {},
        1 => data::download(&cli.libs),
        _ => panic!("ERROR: too many args")
    }

    match cli.download_package {
        0 => {},
        1 => data::download_package(&cli.libs),
        _ => panic!("ERROR: too many args")
    }


    match cli.add {
        0 => {},
        1 => data::add(),
        _ => panic!("ERROR: too many args")
    }

    match cli.remove {
        0 => {},
        1 => data::remove(),
        _ => panic!("ERROR: too many args")
    }


    match cli.list {
        0 => {},
        1 => util::list(),
        _ => panic!("ERROR: too many args")
    }

    match cli.list_package {
        0 => {},
        1 => util::list_package(),
        _ => panic!("ERROR: too many args")
    }


    match cli.install {
        0 => {},
        1 => util::install(),
        _ => panic!("ERROR: too many args")
    }
}
