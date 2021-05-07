use rust_embed::RustEmbed;
use spinner::SpinnerBuilder;
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
    let index_html = YamlRepos::get("yml/repos.yml").unwrap();
    println!("{:?}", std::str::from_utf8(index_html.as_ref()).unwrap());
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
