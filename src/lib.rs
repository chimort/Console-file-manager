use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};


pub struct Config {
    file_option: String,
    first_arg: String,
    second_arg: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_option = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t pick the option"),
        };

        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get first argument"),
        };

        if file_option == String::from("rename") || file_option == String::from("write") {
            let second_arg = match args.next() {
                Some(arg) => Some(arg),
                None => return Err("Didn`t get second argument"),
            };
            Ok(Config { file_option, first_arg, second_arg } )
        }else {
            let second_arg = args.next();
            Ok(Config { file_option, first_arg, second_arg } )
        }
    }
    
    pub fn option_checker(self) -> Result<(), io::Error> {
        match self.file_option.as_ref() {
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
    let initial_title = config.first_arg;
    let final_title = config.second_arg.unwrap();

    fs::rename(format!("{}", initial_title), format!("{}", final_title))?;

    Ok(())
}

fn create_file(config: Config) -> io::Result<()> {
    let file_name = config.first_arg;

    File::create(format!("{}", file_name))?;

    Ok(())
}

fn remove_file(config: Config) -> io::Result<()> {
    let file_name = config.first_arg;

    fs::remove_file(format!("{}", file_name))?;
    Ok(())
}

fn open_file(config: Config) -> io::Result<()> {
    let initital_name = config.first_arg;
    
    let mut file = File::open(format!("{}", initital_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}

fn write_to_file(config: Config) -> io::Result<()> {
    let initial_name = config.first_arg;
    let text = config.second_arg.unwrap();

    let mut file = fs::OpenOptions::new().append(true).open(format!("{}", initial_name))?;
    file.write_all(format!("{}\n", text).as_bytes())?;

    Ok(())

}