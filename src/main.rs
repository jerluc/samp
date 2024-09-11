use argh::FromArgs;
use rand::Rng;
use rand_seeder::{Seeder, SipHasher};
use std::io::{stdin, BufRead, Result};

#[derive(FromArgs)]
/// Sample stdin
struct SampCli {
    #[argh(option, short = 'r', default = "0.5")]
    /// sample ratio
    ratio: f32,

    #[argh(option, short = 's')]
    /// seed string
    seed: Option<String>,
}

fn main() -> Result<()> {
    let args: SampCli = argh::from_env();

    let mut rng = match args.seed {
        Some(seed) => Seeder::from(seed).make_rng(),
        None => SipHasher::new().into_rng(),
    };

    for line in stdin()
        .lock()
        .lines()
        .filter(|_| rng.gen::<f32>() < args.ratio)
    {
        println!("{}", line?)
    }

    Ok(())
}
