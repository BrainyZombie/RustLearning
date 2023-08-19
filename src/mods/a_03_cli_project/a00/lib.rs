use std::error::Error;
use std::fs;
pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let query = args[0].clone();
        let file_name = args[1].clone();
        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    dbg!(contents);
    Ok(())
}
