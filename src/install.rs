pub fn install(pkg: &str) {
    println!("Finding package {}", pkg);
    io_utils::db::find_pkg(pkg);
}
