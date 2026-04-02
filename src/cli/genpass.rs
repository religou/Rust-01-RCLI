use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Length of the password", default_value_t = 16)]
    pub length: u8,

    #[arg(long, help = "Include uppercase letters", default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, help = "Include lowercase letters", default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, help = "Include numbers", default_value_t = true)]
    pub numbers: bool,

    #[arg(long, help = "Include symbols", default_value_t = true)]
    pub symbols: bool,
}
