use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::fs;

fn main() {
    // Read a file in the local file System
    let mut data_file = File::open("data.txt").unwrap();

    // Create an empty mutable String
    let mut file_content = String::new();

    // Cop contets of file to a mutable String
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);

    // try to copy file in other file
    let mut data_file_cp = File::create("data.cp").expect("creation failed");

    // write contetns to the file
    data_file_cp.write(file_content.as_bytes()).expect("write failed");

    println!("Created and copied");

    // use std::fs;
    //let paths = fs::read_dir("./").unwrap();
    let paths = fs::read_dir("/s+c/Azubis/geisler/wochenberichte/src/files/").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().file_name().display())
    }
}
//https://www.programiz.com/rust/file-handling

// Daten aus einer Datei erfolgreich in eine andere Datei gepackt...
// daher dass target nicht getracket wird, data.txt muss in target/debug angelegt werden
