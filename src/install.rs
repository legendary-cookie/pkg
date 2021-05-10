pub fn install(pkg: &str) {
    println!("Finding package {}", pkg);
    let down_url = io_utils::db::find_pkg(pkg);
    if down_url == "" {
        println!("This package doesn't exist. Maybe try running the sync subcommand.");
        return;
    }
    
}
