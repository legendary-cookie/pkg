use std::thread;
use std::time;

use spinner::SpinnerBuilder;

pub fn add(repo: &str) {
    let sp = SpinnerBuilder::new("Checking repo, please wait...".into()).start();
    sp.update(format!("Adding repo: {}", repo));
    for_millis(1000);
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