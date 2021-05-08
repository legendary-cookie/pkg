use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Package {
    package: String,
    url: String,
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
    let testpackage = Package {
        package: "test".to_owned(),
        url: "https://repo.example.com/test".to_owned(),
    };
    conn.execute(
        "INSERT INTO packages (package, url) VALUES (?1, ?2)",
        params![testpackage.package, testpackage.url],
    )
    .unwrap();
}

fn open_pkg_db() -> Connection {
    let path = get_data_dir() + "/data.db3";
    let db = Connection::open(&path);
    return db.unwrap();
}

fn get_data_dir() -> String {
    match home::home_dir() {
        Some(path) => return path.display().to_string() + "/.conf/pkg-cosmo/",
        None => println!("Impossible to get your home dir!"),
    }
    return String::from("");
}
