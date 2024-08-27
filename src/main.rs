use argh::FromArgs;
use rand::Rng;
use rand_seeder::{Seeder, SipHasher};
use std::io::{stdin, BufRead, Result};

#[derive(FromArgs)]
/// Sample stdin
struct SampCli {
    #[argh(option, short = 'r')]
    /// sample ratio
    ratio: Option<f32>,

    #[argh(option, short = 's')]
    /// seed string
    seed: Option<String>,
}

fn main() -> Result<()> {
    let args: SampCli = argh::from_env();

    let sample_ratio = args.ratio.unwrap_or(0.5);

    let mut rng = if let Some(seed) = args.seed {
        Seeder::from(seed).make_rng()
    } else {
        SipHasher::new().into_rng()
    };

    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            let dice: f32 = rng.gen();
            if dice < sample_ratio {
                println!("{}", line);
            }
        } else {
            break;
        }
    }
    Ok(())
}
