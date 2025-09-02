use clap::{Arg, ArgAction, Command};

fn main() {
    // var definieren
    let mut thema = "";


    let matches = Command::new("NoBrainer")
        .version("0.1")
        .author("jethi05")
        .about("WB automatisierung mit Hilfe von internem tool")
        .arg(
            Arg::new("Jahresurlaub")
                .short('J')
                .long("Jahresurlaub")
                .help("Jahresurlaub wird als Thema angegeben")
                .required(false)
                .action(ArgAction::SetTrue), // setzt nur flag
        )
        .arg(
            Arg::new("fehlende_berichte")
                .short('f')
                .long("fehlende_berichte")
                .help("Anzahl fehlender Berichte wird angegeben")
                .required(false)
                .action(ArgAction::SetTrue), // setzt nur flag
        )
        .arg(
            Arg::new("link")
                .short('L')
                .long("link")
                .value_name("link")
                .help("Legt einen HTML Link an in das Dokument")
                .required(false)
                .num_args(1),
        )
        .get_matches();


    if matches.get_flag("Jahresurlaub") {
        thema = "Jahresurlaub";
    }

    if matches.get_flag("fehlende_berichte") {
        fehlende_berichte();
    }

    if let Some(link) = matches.get_one::<String>("link") {
        println!("Link wurde eingegeben: {}", link);
    }
        
}

fn fehlende_berichte() {
    println!("Funktion fehlende_berichte wird ausgefüht");
}
