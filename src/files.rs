pub mod file_func {
    use std::{fs::{File, self}, io::{self, Read, Write}};

    use crate::Config;

    
    pub fn rename_file(config: Config) -> io::Result<()> {
        let initial_title = config.first_arg;
        let final_title = config.second_arg.unwrap();
    
        fs::rename(format!("{}", initial_title), format!("{}", final_title))?;
    
        Ok(())
    }
    
    pub fn create_file(config: Config) -> io::Result<()> {
        let file_name = config.first_arg;
    
        File::create(format!("{}", file_name))?;
    
        Ok(())
    }
    
    pub fn remove_file(config: Config) -> io::Result<()> {
        let file_name = config.first_arg;
    
        fs::remove_file(format!("{}", file_name))?;
        Ok(())
    }
    
    pub fn open_file(config: Config) -> io::Result<()> {
        let initital_name = config.first_arg;
        
        let mut file = File::open(format!("{}", initital_name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        println!("{}", contents);
    
        Ok(())
    }
    
    pub fn write_to_file(config: Config) -> io::Result<()> {
        let initial_name = config.first_arg;
        let text = config.second_arg.unwrap();
    
        let mut file = fs::OpenOptions::new().append(true).open(format!("{}", initial_name))?;
        file.write_all(format!("{}\n", text).as_bytes())?;
    
        Ok(())
    
    }
    
    
    pub fn copy(config: Config) -> io::Result<()> {
        let from = config.first_arg;
        let to = config.second_arg.unwrap();
    
        fs::copy(format!("{}", from), format!("{}", to))?;
        Ok(())
    }
}