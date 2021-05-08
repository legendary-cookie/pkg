use io_utils;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use tempfile::tempfile;
use yaml_rust::{YamlEmitter, YamlLoader};

pub fn sync() {
    io_utils::db::clear_db();
    let file = File::open(io_utils::get_conf_folder() + "/repos.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut packages = vec![];
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut tmp_file = tempfile().unwrap();
        let mut resp = reqwest::get(line).expect("request failed");
        std::io::copy(&mut resp, &mut out).expect("failed to copy content");
        //writeln!(tmp_file, "{}", line).unwrap();
    }
    io_utils::db::insert_pkg(packages)
}
