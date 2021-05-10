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

pub fn clear_db() {
    let conn = open_pkg_db();
    conn.execute("DROP TABLE IF EXISTS packages", []).ok();
    conn.close().ok();
    init_db();
}
