use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

mod formatter;
use formatter::*;

/// Returns a buildstamp, like `23W42.12345`
///
/// Useful for tagging builds and images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
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
    /// always uppercase unless `--lowercase` is used.
    ///
    /// Note: You can also use it to put arbitrary text into the buildstamp. ;-)
    #[arg(
        short,
        long,
        env,
        default_value_t = String::new(),
        default_missing_value = "A",
        num_args(0..=1),
        ignore_case(true)
    )]
    minecraft: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut stamp = time::OffsetDateTime::now_utc().format(args.format.formatter())?;
    if args.format == Format::Weekly {
        stamp.push_str(&args.minecraft.to_ascii_uppercase())
    }
    if args.lowercase {
        stamp = stamp.to_ascii_lowercase();
    }
    if args.newline {
        stamp.push('\n');
    }
    print!("{stamp}");
    Ok(())
}
