use clap::{App, Arg};

fn main() {
    let matches = App::new("pkg-cosmo")
        .about("A package manager nobody asked for")
        .version("1.0")
        .author("Vento")
        .subcommand(
            App::new("repo-add")
                .about("adds a new repository")
                .about("Add a repo")
                .arg(
                    Arg::new("URL")
                        .about("repository URL to add")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("repo-add") {
        // Now we have a reference to clone's matches
        let repo = add_matches.value_of("URL").unwrap();
        println!("Adding repo: {}", repo);
    }
}
