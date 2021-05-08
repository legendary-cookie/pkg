mod install;
mod repo_manage;
use clap::{App, Arg};
use io_utils;

fn main() {
    io_utils::setup_files();
    let matches = App::new("pkg-cosmo")
        .about("A package manager nobody asked for")
        .version("1.0")
        .author("Vento")
        .subcommand(
            App::new("repo").about("All the repo stuff").subcommand(
                App::new("add").about("Add a repo").arg(
                    Arg::new("URL")
                        .about("repository URL to add")
                        .required(true),
                ),
            ),
        )
        .subcommand(
            App::new("install").about("Install a package").arg(
                Arg::new("PACKAGE")
                    .about("Name of the package")
                    .required(true),
            ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("repo") {
        if let Some(add_matches) = matches.subcommand_matches("add") {
            let repo = add_matches.value_of("URL").unwrap();
            repo_manage::add(repo);
        }
    }

    if let Some(install_matches) = matches.subcommand_matches("install") {
        let package = install_matches.value_of("PACKAGE").unwrap();
        install::install(package);
    }
}
