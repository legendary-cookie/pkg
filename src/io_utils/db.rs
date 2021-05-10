use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Package {
    pub package: String,
    pub url: String,
}

pub fn init_db() {
    let conn = open_pkg_db();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
                  package              TEXT NOT NULL,
                  url            TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS installed (
                  package              TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn.close().ok();
}

fn open_pkg_db() -> Connection {
    let path = get_data_dir() + "/data.db3";
    let db = Connection::open(&path);
    return db.unwrap();
}

pub fn get_data_dir() -> String {
    match home::home_dir() {
        Some(path) => return path.display().to_string() + "/.pkgs/",
        None => println!("Impossible to get your home dir!"),
    }
    return String::from("");
}

#[allow(deprecated)]
pub fn find_pkg(pkg: &str) -> String {
    let conn = open_pkg_db();
    let mut stmt = conn
        .prepare("SELECT url FROM packages WHERE package = ?1")
        .unwrap();
    let mut rows = stmt.query([pkg]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        return row.get_raw(0).as_str().unwrap().to_string();
    }
    return "".to_owned();
}

pub fn insert_pkg(pkgs: std::vec::Vec<Package>) {
    let conn = open_pkg_db();
    for pkg in pkgs {
        conn.execute(
            "INSERT INTO packages (package, url) VALUES (?1, ?2)",
            params![pkg.package, pkg.url],
        )
        .unwrap();
    }
    conn.close().ok();
}

pub fn insert_installed(pkg: String) {
    let conn = open_pkg_db();
    conn.execute("INSERT INTO installed (package) VALUES (?1)", params![pkg])
        .unwrap();
    conn.close().ok();
}

pub fn remove_installed(pkg: String) {
    let conn = open_pkg_db();
    conn.execute("DELETE FROM installed WHERE package = ?1", params![pkg])
        .unwrap();
    conn.close().ok();
}

#[allow(deprecated)]
pub fn print_installed() {
    let conn = open_pkg_db();
    let mut stmt = conn.prepare("SELECT package FROM installed").unwrap();
    let mut rows = stmt.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        println!("\t{}", row.get_raw(0).as_str().unwrap())
    }
}

pub fn clear_db() {
    let conn = open_pkg_db();
    conn.execute("DROP TABLE IF EXISTS packages", []).ok();
    conn.close().ok();
    init_db();
}
