use io_utils;
use io_utils::db::Package;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

pub fn sync() {
    let file = File::open(io_utils::get_conf_folder() + "repos.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut packages = vec![];
    for line in file.lines().filter_map(|result| result.ok()) {}
    let test = Package {
        package: "test".to_owned(),
        url: "test.com".to_owned(),
    };
    packages.push(&test);
    packages.push(&test);
    packages.push(&test);
    packages.push(&test);
    packages.push(&test);
    packages.push(&test);
    
    io_utils::db::insert_pkg(packages)
}
