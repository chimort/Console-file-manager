use std:: {env, process};
use cli_application::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem with parsing argumenst: {err}");
        process::exit(1);
    });

    if let Err(e) = Config::goal_checker(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
