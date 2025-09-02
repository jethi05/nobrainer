use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON-Datei öffnen
    let file = File::open("file.json")?;
    let reader = BufReader::new(file);

    // Deserialisieren in struct
    let config: Config = serde_json::from_reader(reader)?;

    // Pfad verwenden
    println!("Der eingelesene Pfad lautet: {}", config.path);

    Ok(())
}

