use pico_args::Arguments as Argumentoj;

use rand::rngs::SmallRng as Hazardileto;
use rand::SeedableRng;

use rand::prelude::SliceRandom;

const HELPO: &str = r#"
UZADO:
    proverbilo [nombro]

ARGOJ:
    <nombro>    Nombro da proverboj montri [defaŭlto: 1]

FLAGOJ:
    -h, --helpo      Printas help-informon.
"#;

include!(concat!(env!("OUT_DIR"), "/proverbaro.rs"));

fn parsu_argumentojn() -> usize {
    let mut argumentoj = Argumentoj::from_env();

    // Kontrolu ĉu la uzanto petis la helpon.
    if argumentoj.contains(["-h", "--helpo"]) {
        print!("{}", HELPO);
        std::process::exit(0);
    }

    match argumentoj.opt_free_from_str() {
        // Akiru nombron el uzanto, aŭ nur printu unu proverbon.
        Ok(eble_nombro) => eble_nombro.unwrap_or(1),
        // Eraro okazis.
        Err(_) => {
            eprintln!("eraro parsante argumento(j)n");
            std::process::exit(1);
        }
    }
}

fn main() {
    // Akiri la nombron da proverbo printi el la uzanto.
    let nombro = parsu_argumentojn();

    // Elekti hazarda(j)n proverbo(j)n.
    let proverboj_printotaj = PROVERBARO.choose_multiple(&mut Hazardileto::from_entropy(), nombro);

    // Printi la proverbojn.
    for proverbo in proverboj_printotaj {
        println!("{}", proverbo);
    }
}
