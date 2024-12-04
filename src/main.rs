use clap::{arg, Parser};
use clap_derive::Parser;
use passgenlib::Passgen;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Length of result.
    #[arg(default_value_t = 8)]
    length: u32,
    /// User defined character set.
    /// ⚠️ This set of characters will exclude all other rules.
    #[arg(short, long, default_value = "")]
    custom_charset: String,
}

fn main() {
    let args = Args::parse();

    if args.custom_charset != "" {
        println!(
            "{}",
            Passgen::default_strong_and_usab().generate(args.length)
        );
    } else {
        println!(
            "{}",
            Passgen::new()
                .set_custom_charset(args.custom_charset.leak())
                .generate(args.length)
        );
    }
}
