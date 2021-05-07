use rust_embed::RustEmbed;
use spinner::SpinnerBuilder;
use std::fs;
use std::path::Path;
use std::thread;
use std::time;
use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(RustEmbed)]
#[folder = "resources/"]
#[prefix = "yml/"]
pub struct YamlRepos;

pub fn add(repo: &str) {
    let sp = SpinnerBuilder::new("Checking repo, please wait...".into()).start();
    sp.update(format!("Adding repo: {}", repo));
    for_millis(1000);
    if !exists(&get_conf_folder()) {
        fs::create_dir_all(get_conf_folder())
            .expect_err("Whilst creating the config dir, something went wrong!");
    }
    if !exists(&(get_conf_folder() + "/repos.yml")) {
        let resource = YamlRepos::get("yml/repos.yml").unwrap();
        let template = std::str::from_utf8(resource.as_ref()).unwrap();
        fs::write(&(get_conf_folder() + "/repos.yml"), template)
            .expect_err("Whilst copying the repos.yml template something went wrong!");
    }
    let repos_file_as_string = fs::read_to_string(&(get_conf_folder() + "/repos.yml")).unwrap();
    let docs = YamlLoader::load_from_str(&repos_file_as_string).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];
    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}",out_str);
    println!("\nAdded repo");
    sp.close();
}

/**
 * Sleep for milliseconds
 */
fn for_millis(duration: u64) {
    let millis = time::Duration::from_millis(duration);
    let now = time::Instant::now();
    thread::sleep(millis);
    assert!(now.elapsed() >= millis);
}

fn get_conf_folder() -> String {
    return String::from("/home/vincent/.conf/pkg-cosmo/");
}
fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
