use io_utils;
use std::fs::File;
use std::io::{BufRead, BufReader};
use yaml_rust::YamlLoader;

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
/*
        let docs = YamlLoader::load_from_str(&body).unwrap();
        let doc = &docs[0];
        let packs = &doc["packages"];
*/
        let doc = serde_yaml::from_str(&body);
        println!("{:?}", doc);
    }
    io_utils::db::insert_pkg(packages)
}
