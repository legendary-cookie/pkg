use std::fs;

pub fn uninstall(pkg: &str) {
    let pkgpath = io_utils::get_bin_dir() + "/" + &pkg;
    fs::remove_file(pkgpath).unwrap();
    io_utils::db::remove_installed(pkg.to_owned());
}
