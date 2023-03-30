use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};


pub struct Config {
    goal: String,
    initial_title: String,
    final_title: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let goal = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t pick the option"),
        };

        let initial_title = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get first argument"),
        };

        if goal == String::from("rename") || goal == String::from("write") {
            let final_title = match args.next() {
                Some(arg) => Some(arg),
                None => return Err("Didn`t get second argument"),
            };
            Ok(Config { goal, initial_title, final_title } )
        }else {
            let final_title = args.next();
            Ok(Config { goal, initial_title, final_title } )
        }
    }
    
    pub fn option_checker(self) -> Result<(), io::Error> {
        match self.goal.as_ref() {
            "rename" => rename_file(self),
            "create" => create_file(self),
            "remove" => remove_file(self),
            "open" => open_file(self),
            "write" => write_to_file(self),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Wrong option"))
        }
    }
}

fn rename_file(config: Config) -> io::Result<()> {
    let initial_title = config.initial_title;
    let final_title = config.final_title.unwrap();

    fs::rename(format!("{}", initial_title), format!("{}", final_title))?;

    Ok(())
}

fn create_file(config: Config) -> io::Result<()> {
    let file_name = config.initial_title;

    File::create(format!("{}", file_name))?;

    Ok(())
}

fn remove_file(config: Config) -> io::Result<()> {
    let file_name = config.initial_title;

    fs::remove_file(format!("{}", file_name))?;
    Ok(())
}

fn open_file(config: Config) -> io::Result<()> {
    let initital_name = config.initial_title;
    
    let mut file = File::open(format!("{}", initital_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}

fn write_to_file(config: Config) -> io::Result<()> {
    let initial_name = config.initial_title;
    let text = config.final_title.unwrap();

    let mut file = fs::OpenOptions::new().append(true).open(format!("{}", initial_name))?;
    file.write_all(format!("{}\n", text).as_bytes())?;

    Ok(())

}