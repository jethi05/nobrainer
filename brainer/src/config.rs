use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub test_var: String,
    pub safe_files: String,
    pub template_files: String,
}


pub fn config_einlesen() -> Result<Config, Box<dyn std::error::Error>> {
    println!("Config wird eingelesen");
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;
    //println!("Test_var: {}", config.test_var);
    //println!("safe_files: {}", config.safe_files);
    //println!("template_files: {}", config.template_files);
    Ok(config)
}
