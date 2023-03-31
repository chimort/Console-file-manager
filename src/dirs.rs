pub mod dir_funcs {
    use std::{io, fs};
    use std::fs::DirBuilder;
    use crate::Config;


    pub fn create_dir(config: Config) -> io::Result<()> {
        let path = config.first_arg;
    
        DirBuilder::new().recursive(true).create(format!("{}", path))?;
    
        Ok(())
    }
    
    pub fn remove_dir(config: Config) -> io::Result<()> {
        let dir = config.first_arg;
    
        fs::remove_dir_all(format!("{}", dir))?;
    
        Ok(())
    }
    
    pub fn read_dir(config: Config) -> io::Result<()> {
        let path = config.first_arg;
    
        let mut entries = fs::read_dir(format!("{}", path))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
    
            entries.sort();
            println!("{:?}", entries);
    
            Ok(())
    }
}