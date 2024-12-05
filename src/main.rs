use clap::{arg, Parser};
use clap_derive::Parser;
use passgenlib::Passgen;

#[derive(Parser, Debug)]
#[command(
    version,
    verbatim_doc_comment,
    about = "\n\n⚙️ Cross-platform tool for generating cryptographically secure passwords/tokens and other sets and sequences."
)]
struct Args {
    /// Length of result.
    #[arg(default_value_t = 8)]
    length: u32,

    /// Include lowercase letters to the generator.
    #[arg(short = 'l', long, default_value = "false")]
    pub letters_on: bool,

    /// Include capital letters to the generator.
    #[arg(short = 'L', long, default_value = "false")]
    pub u_letters_on: bool,

    /// Include numeric characters to the generator.
    #[arg(short = 'n', long, default_value = "false")]
    pub nums_on: bool,

    /// Include special characters to the generator.
    #[arg(short = 's', long, default_value = "false")]
    pub spec_symbs_on: bool,

    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters "0oOiIlL1".
    /// ⚠️If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule "custom_charset".
    #[arg(short = 'S', long, default_value = "false", verbatim_doc_comment)]
    pub strong_usab_on: bool,

    /// User defined character set.
    /// ⚠️This set of characters will exclude all other rules except for a rule "strong_usab_on".
    /// ⚙️If "strong_usab_on" on too then you can generate combined strong and usability result with custom charset.
    #[arg(short, long, default_value = "", verbatim_doc_comment)]
    custom_charset: String,
}

fn main() {
    let args: Args = Args::parse();

    fn is_ruleset_clean(args: &Args) -> bool {
        args.custom_charset == ""
            && !args.letters_on
            && !args.u_letters_on
            && !args.nums_on
            && !args.spec_symbs_on
            && !args.strong_usab_on
    }

    if is_ruleset_clean(&args) {
        println!("{}", Passgen::default().generate(args.length));
    } else {
        let mut pgen = Passgen {
            enab_letters: args.letters_on,
            enab_u_letters: args.u_letters_on,
            enab_num: args.nums_on,
            enab_spec_symbs: args.spec_symbs_on,
            enab_strong_usab: args.strong_usab_on,
            custom_charset: args.custom_charset.leak(),
        };
        println!("{}", pgen.generate(args.length));
    }
}
