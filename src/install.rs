use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

pub fn install(pkg: &str) {
    // Finding the package
    println!("Finding package {}", pkg);
    let down_url = io_utils::db::find_pkg(pkg);
    if down_url == "" {
        println!("This package doesn't exist. Maybe try running the sync subcommand.");
        return;
    }
    // Download part
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(&down_url).unwrap();
    let _redirect = easy.follow_location(true);
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    {
        let pkgpath = io_utils::get_bin_dir() + "/" + &pkg;
        let mut file = File::create(&pkgpath).unwrap();
        file.write_all(dst.as_slice()).unwrap();
        fs::set_permissions(&pkgpath, fs::Permissions::from_mode(0o755)).unwrap();
    }
}
