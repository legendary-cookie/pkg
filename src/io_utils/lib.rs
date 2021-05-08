use rust_embed::RustEmbed;
use std::fs;
use std::thread;
use std::time;

#[derive(RustEmbed)]
#[folder = "resources/"]
#[prefix = "assets/"]
pub struct YamlRepos;

/**
 * Sleep for milliseconds
 */
pub fn for_millis(duration: u64) {
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
    if !exists(&(get_conf_folder() + "/repos.txt")) {
        let resource = YamlRepos::get("assets/repos.txt").unwrap();
        let template = std::str::from_utf8(resource.as_ref()).unwrap();
        fs::write(&(get_conf_folder() + "/repos.txt"), template)
            .expect_err("Whilst copying the repos.txt template something went wrong!");
    }
}
