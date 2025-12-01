use time::{format_description::BorrowedFormatItem, macros::format_description as fd};

// https://man7.org/linux/man-pages/man1/date.1.html
// STAMP = $(shell bash -c 'date +%gW%V.%w%H%M') # 22W50.12345
// https://time-rs.github.io/book/api/format-description.html

type Formatter = &'static [BorrowedFormatItem<'static>];

const WEEKLY: Formatter = fd!("[year base:iso_week repr:last_two]W[week_number repr:iso]");

const ON_MINUTE: Formatter = fd!("\
    [year base:iso_week repr:last_two]W[week_number repr:iso].\
    [weekday repr:monday][hour repr:24][minute]");

const ON_SECOND: Formatter = fd!("\
    [year base:iso_week repr:last_two]W[week_number repr:iso].\
    [weekday repr:monday][hour repr:24][minute][second]");

const ON_MILLIS: Formatter = fd!("\
    [year base:iso_week repr:last_two]W[week_number repr:iso].\
    [weekday repr:monday][hour repr:24][minute][second].[subsecond digits:3]");

const HIGH_PREC: Formatter = fd!("\
    [year base:iso_week repr:last_two]W[week_number repr:iso].\
    [weekday repr:monday][hour repr:24][minute][second].[subsecond digits:9]");

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Format {
    #[default]
    Weekly,
    OnMinute,
    OnSecond,
    OnMillis,
    HighPrec,
}

impl Format {
    pub const VALUES: [&'static str; 5] = ["weekly", "minute", "second", "millis", "highprec"];

    pub fn formatter(&self) -> Formatter {
        match self {
            Format::Weekly => WEEKLY,
            Format::OnMinute => ON_MINUTE,
            Format::OnSecond => ON_SECOND,
            Format::OnMillis => ON_MILLIS,
            Format::HighPrec => HIGH_PREC,
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Format::Weekly => "weekly",
            Format::OnMinute => "minute",
            Format::OnSecond => "second",
            Format::OnMillis => "millis",
            Format::HighPrec => "highprec",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "weekly" => Ok(Format::Weekly),
            "minute" => Ok(Format::OnMinute),
            "second" => Ok(Format::OnSecond),
            "millis" => Ok(Format::OnMillis),
            "highprec" => Ok(Format::HighPrec),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}
