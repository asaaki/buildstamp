use buildstamp::{Buildstamp, Format};
use clap::Parser;
use clap::builder::{PossibleValuesParser, TypedValueParser};

/// Returns a buildstamp, like `23W42.12345`
///
/// Useful for tagging builds and images
#[derive(Parser, Debug)]
#[command(name = "buildstamp", author, version, about, long_about)]
struct Args {
    /// Which format to use for the buildstamp
    ///
    #[arg(
        value_parser = PossibleValuesParser::new(Format::VALUES).map(|s| s.parse::<Format>().unwrap()),
        default_value_t = Format::Weekly,
        ignore_case = true
    )]
    format: Format,

    /// Add newline to the output
    #[arg(short, long, default_value_t = false)]
    newline: bool,

    /// Print all letters lowercased
    #[arg(short, long, default_value_t = false)]
    lowercase: bool,

    /// Specify the Minecraft build iteration (A, B, C, ...)
    ///
    /// Works only with the "weekly" format and appends the content of this option;
    /// always uppercase unless `--lowercase` is used. Will use "A" if no value is given.
    ///
    /// Note: You can also use it to put arbitrary text into the buildstamp. ;-)
    #[arg(
        short,
        long,
        env,
        default_missing_value = "A",
        num_args(0..=1),
        ignore_case(true)
    )]
    minecraft: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let stamp =
        Buildstamp::new_from_args(args.format, args.newline, args.lowercase, args.minecraft);
    stamp.print();
    Ok(())
}
