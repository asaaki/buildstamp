use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

mod formatter;
use formatter::*;

/// Returns a buildstamp, like 23W42.12345
///
/// Useful for tagging builds and images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Which format to use for the buildstamp
    #[arg(default_value_t = Format::Weekly,
        value_parser = PossibleValuesParser::new(Format::VALUES).map(|s| s.parse::<Format>().unwrap()),
    )]
    format: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let stamp = time::OffsetDateTime::now_utc().format(args.format.formatter())?;

    print!("{stamp}");
    Ok(())
}
