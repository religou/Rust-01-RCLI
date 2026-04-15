use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_sign, process_verify,
    Base64SubCommand, Opts, Subcommand, TextSignFormat, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
        }
        Subcommand::Base64(base64_cmd) => match base64_cmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        Subcommand::Text(text_sub_command) => match text_sub_command {
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::BLAKE3 => {
                    process_sign(&opts.input, &opts.key, opts.format)?;
                }
                TextSignFormat::ED25519 => {
                    println!("ED25519 signing is not implemented yet");
                }
            },
            TextSubCommand::Verify(opts) => match opts.format {
                TextSignFormat::BLAKE3 => {
                    process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                }
                TextSignFormat::ED25519 => {
                    println!("ED25519 verification is not implemented yet");
                }
            },
        },
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
