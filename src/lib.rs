use std::fs;
use std::fs::File;


pub struct Config {
    pub goal: String,
    pub initial_title: String,
    pub final_title: Option<String>,
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
            None => return Err("Didn`t get initial file name"),
        };

        if goal == String::from("rename") {
            let final_title = match args.next() {
                Some(arg) => Some(arg),
                None => return Err("Didn`t get final file name"),
            };
            Ok(Config { goal, initial_title, final_title } )
        }else {
            let final_title = args.next();
            Ok(Config { goal, initial_title, final_title } )
        }
    }
    
    pub fn goal_checker(self) -> std::io::Result<()> {
        match self.goal.as_ref() {
            "rename" => rename_file(self),
            "create" => create_file(self),
            _ => panic!("There is no such option"),
        }
    }
}

pub fn rename_file(config: Config) -> std::io::Result<()> {
    let initial_title = config.initial_title;
    let final_title = config.final_title.unwrap();

    fs::rename(format!("{}", initial_title), format!("{}", final_title))?;

    Ok(())
}

pub fn create_file(config: Config) -> std::io::Result<()> {
    let file_name = config.initial_title;

    File::create(format!("{}", file_name))?;

    Ok(())
}