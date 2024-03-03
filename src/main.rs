use clap::Parser;
mod data;
mod util;

#[derive(Parser)]
struct Cli {
    libs: Vec<String>,

    #[arg(short, action = clap::ArgAction::Count, help = "Add it to other args to get the package version")]
    package: u8,

    #[arg(short, long, action = clap::ArgAction::Count, help = "Downloads libraries from stored files")]
    download: u8,

    #[arg(short, long, action = clap::ArgAction::Count, help = "Adds a library to the stored file")]
    add: u8,

    #[arg(short, long, action = clap::ArgAction::Count, help = "removes a library from the stored files")]
    remove: u8,

    #[arg(short, long, action = clap::ArgAction::Count, help = "lists the libraries stored")]
    list: u8,

    #[arg(long, action = clap::ArgAction::Count, help = "installs the programme locally")]
    install: u8,

    #[arg(long, action = clap::ArgAction::Count, help = "uninstalls the programme")]
    uninstall: u8
}

fn main() {
    let cli = Cli::parse();

    match cli.package {
        0 => {
            match cli.download {
                0 => {}
                _ => data::download(&cli.libs)
            }

            match cli.add {
                0 => {}
                _ => data::add()
            }

            match cli.remove {
                0 => {}
                _ => data::remove()
            }

            match cli.list {
                0 => {}
                _ => util::list()
            }

            match cli.install {
                0 => {}
                _ => util::install()
            }

            match cli.uninstall {
                0 => {}
                _ => util::uninstall()
            }
        },
        _ => {
            match cli.download {
                0 => {}
                _ => data::download_package(&cli.libs)
            }

            match cli.add {
                0 => {}
                _ => data::add_package()
            }

            match cli.remove {
                0 => {}
                _ => data::remove_package()
            }

            match cli.list {
                0 => {}
                _ => util::list_package()
            }
        }
    }
}
