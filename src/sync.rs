use io_utils;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn sync() {
    let file = File::open(io_utils::get_conf_folder() + "repos.txt").expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
       
    }
}
