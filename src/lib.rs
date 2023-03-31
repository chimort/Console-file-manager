use std::io;

mod dirs;
mod files;

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

        if file_option == String::from("rename") || file_option == String::from("write") || file_option == String::from("copy") {
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
            "rename" => files::file_func::rename_file(self),
            "create" => files::file_func::create_file(self),
            "remove" => files::file_func::remove_file(self),
            "open" => files::file_func::open_file(self),
            "write" => files::file_func::write_to_file(self),
            "copy" => files::file_func::copy(self),
            "cdir" => dirs::dir_funcs::create_dir(self),
            "rdir" => dirs::dir_funcs::remove_dir(self),
            "odir" => dirs::dir_funcs::read_dir(self),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Wrong option"))
        }
    }
}
