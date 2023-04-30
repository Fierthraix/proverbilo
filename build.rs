use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Konservu liston al disko por aliaj funkcioj.
    let ĝenita_dosiervojo = Path::new(&env::var("OUT_DIR").unwrap()).join("proverbaro.rs");
    let mut dosiero = BufWriter::new(File::create(ĝenita_dosiervojo).unwrap());

    // Elŝarĝu frazojn el tekstdosiero.
    let proverbaro: Vec<String> = BufReader::new(File::open("proverbaro.txt")?)
        .lines()
        .map(|p| p.unwrap())
        .collect();

    writeln!(
        dosiero,
        "const PROVERBARO: [&str; {}] = [",
        proverbaro.len()
    )?;
    for proverbo in &proverbaro {
        writeln!(dosiero, r#"    {:?},"#, proverbo)?;
    }
    writeln!(dosiero, "];")?;

    println!("{:?}", proverbaro);

    Ok(())
}
