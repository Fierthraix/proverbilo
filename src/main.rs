use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Argumentujo {
    #[structopt(default_value = "1")]
    /// Nombro da proverboj montri.
    nombro: usize,
}

fn main() {
    // Akiri proverbojn el tekstdosiero.
    let proverbaro = include_str!("../proverbaro.txt").split('\n');

    // Atingi argumentojn el la uzanto.
    let argumentoj = Argumentujo::from_args();

    // Elekti hazarda(j)n proverbo(j)n.
    let proverboj_printotaj =
        proverbaro.choose_multiple(&mut SmallRng::from_entropy(), argumentoj.nombro);

    // Printi la proverbojn.
    for proverbo in proverboj_printotaj {
        println!("{}", proverbo);
    }
}
