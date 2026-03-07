// rcli csv -i input.csv -o output.json -d ',' --header
use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts {
        Opts {
            cmd: Subcommand::Csv(opts),
        } => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}

// #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
