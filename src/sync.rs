use io_utils;
use tempfile::tempfile;
use tempfile::NamedTempFile;
use std::fs::File;
use std::io::{BufRead, self, Write, BufReader};

pub fn sync() {
    io_utils::db::clear_db();
    let file = File::open(io_utils::get_conf_folder() + "/repos.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut packages = vec![];
    for line in file.lines().filter_map(|result| result.ok()) {
        if line.starts_with("\n") {
            continue;
        }
        if !(line.starts_with("https://") || line.starts_with("http://")) {
            continue;
        }
        let body: String = ureq::get(&line)
            .call().unwrap()
            .into_string().unwrap();
        let mut file = NamedTempFile::new().unwrap();
        let mut file2 = file.reopen().unwrap();

        writeln!(file, "{}", body).unwrap();
        let read = BufReader::new(file2);
        
        for line in read.lines().filter_map(|result| result.ok()) {
            if line.starts_with("\n") {
                continue;
            }
            if !(line.contains("https://") || line.starts_with("http://")) {
                continue;
            }
            let mut pkgname = vec![];
            for c in line.chars() {
                pkgname.push(c);
                if c == '|' {
                    break;
                }
            }
            println!("{:?}", pkgname);
        }
    }
    io_utils::db::insert_pkg(packages)
}
