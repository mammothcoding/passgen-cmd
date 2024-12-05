use clap::{arg, Parser};
use clap_derive::Parser;
use passgenlib::Passgen;

#[derive(Parser, Debug)]
#[command(version, verbatim_doc_comment, about = "\n\n⚙️ Cross-platform tool for generating cryptographically secure passwords/tokens and other sets and sequences.")]
struct Args {
    /// Length of result.
    #[arg(default_value_t = 8)]
    length: u32,

    /// Include lowercase letters to the generator.
    #[arg(short = 'l', long, default_value = "false")]
    pub letters_on: bool,

    /// Include capital letters to the generator.
    #[arg(short = 'L', long, default_value = "false")]
    pub enab_u_letters: bool,

    /// Include numeric characters to the generator.
    #[arg(short = 'n', long, default_value = "false")]
    pub enab_num: bool,

    /// Include special characters to the generator.
    #[arg(short = 's', long, default_value = "false")]
    pub enab_spec_symbs: bool,

    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters "0oOiIlL1".
    /// ⚠️If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule "custom_charset".
    #[arg(short = 'S', long, default_value = "false", verbatim_doc_comment)]
    pub enab_strong_usab: bool,

    /// User defined character set.
    /// ⚠️This set of characters will exclude all other rules.
    #[arg(short, long, default_value = "", verbatim_doc_comment)]
    custom_charset: String,
}

fn main() {
    let args = Args::parse();

    if args.custom_charset == "" {
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
