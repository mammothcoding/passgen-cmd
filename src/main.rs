use passgenlib::Passgen;
use clap::{arg, command, value_parser, Arg, ColorChoice, Parser};

/*#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}*/

fn main() {



    let matches = command!()
        .arg(
            arg!([length])
                .value_parser(value_parser!(u32))
                .default_value("8"),
        )
        .color(ColorChoice::Always)
        .get_matches();

    let length: &u32 = matches
        .get_one::<u32>("length")
        .expect("length must be provided");

    println!("{}", Passgen::default().generate(*length));


}
