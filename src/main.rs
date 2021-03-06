mod install;
mod repo_manage;
mod sync;
mod uninstall;
use clap::{App, Arg};
use io_utils;
use io_utils::db;

fn main() {
    io_utils::setup_files();
    db::init_db();
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
        .subcommand(App::new("sync").about("Sync with package repos"))
        .subcommand(
            App::new("uninstall").about("Uninstall a package").arg(
                Arg::new("PACKAGE")
                    .about("Package to remove")
                    .required(true),
            ),
        )
        .subcommand(
            App::new("install").about("Install a package").arg(
                Arg::new("PACKAGE")
                    .about("Name of the package")
                    .required(true),
            ),
        )
        .subcommand(App::new("list").about("List installed packages"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("repo") {
        if let Some(add_matches) = matches.subcommand_matches("add") {
            let repo = add_matches.value_of("URL").unwrap();
            repo_manage::add(repo);
        }
    } else if let Some(_sync_matches) = matches.subcommand_matches("sync") {
        sync::sync();
    } else if let Some(install_matches) = matches.subcommand_matches("install") {
        let package = install_matches.value_of("PACKAGE").unwrap();
        install::install(package);
    } else if let Some(uninstall_matches) = matches.subcommand_matches("uninstall") {
        let package = uninstall_matches.value_of("PACKAGE").unwrap();
        uninstall::uninstall(package);
    } else if let Some(_list_matches) = matches.subcommand_matches("list") {
        println!("You have the following packages installed: ");
        io_utils::db::print_installed();
    } else {
        println!("Run with the --help flag to see a full list of available commands!");
    }
}
