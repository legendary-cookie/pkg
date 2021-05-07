use rust_embed::RustEmbed;
use spinner::SpinnerBuilder;
use std::collections::BTreeMap;
use std::fs;
use std::thread;
use std::time;

#[derive(RustEmbed)]
#[folder = "resources/"]
#[prefix = "yml/"]
pub struct YamlRepos;

pub fn add(repo: &str) {
    let sp = SpinnerBuilder::new("Checking repo, please wait...".into()).start();
    sp.update(format!("Adding repo: {}", repo));
    setup_files();
    let repos_file_as_string = fs::read_to_string(&(get_conf_folder() + "/repos.yml")).unwrap();
    sp.update(format!("Reading repos"));
    for_millis(500);
    let value: serde_yaml::Value = serde_yaml::from_str(&repos_file_as_string).unwrap();
    sp.update(format!("Appending the new repo"));
    for_millis(500);
    let mut map = BTreeMap::new();
    let mut repos_lol: Vec<String> = Vec::new();
    for repo_lol in value["repos"].as_sequence().unwrap() {
        repos_lol.push(serde_yaml::to_string(repo_lol).unwrap());
    }
    map.insert("repos", &repos_lol);

    println!("{:?}", map);
    println!("{:?}", repos_lol);
    //let repos: serde_yaml::Value = serde_yaml::from_str(&new_repos).unwrap();
    //serde_yaml::to_writer(std::io::stdout(), &repos).unwrap();
    let mapval = serde_yaml::to_value(&map).unwrap();
    serde_yaml::to_writer(std::io::stdout(), &mapval).unwrap();

    serde_yaml::to_writer(std::io::stdout(), &value).unwrap();

    println!("\nAdded repo successfully!");
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

pub fn get_conf_folder() -> String {
    return String::from("/home/vincent/.conf/pkg-cosmo/");
}
pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
pub fn setup_files() {
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
}
