use io_utils;
use rust_embed::RustEmbed;
use spinner::SpinnerBuilder;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(RustEmbed)]
#[folder = "resources/"]
#[prefix = "assets/"]
pub struct YamlRepos;

pub fn add(repo: &str) {
    let sp = SpinnerBuilder::new("Checking repo, please wait...".into()).start();
    sp.update(format!("Adding repo: {}", repo));
    sp.update(format!("Reading repos"));
    io_utils::for_millis(500);

    sp.update(format!("Appending the new repo"));
    io_utils::for_millis(500);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(io_utils::get_conf_folder() + "/repos.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", repo) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("\nAdded repo successfully!");
    sp.close();
}
