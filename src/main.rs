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
    include_str!("../proverbaro.txt")
        .split('\n')
        .choose_multiple(&mut SmallRng::from_entropy(), Argumentujo::from_args().nombro)
        .iter()
        .filter(|proverbo| {
            println!("{}", proverbo);
            false
        }).next();
}
