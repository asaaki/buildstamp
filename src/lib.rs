mod formatter;
pub use formatter::*;

/// Holds information about how to output the buildstamp
pub struct Buildstamp {
    timestamp: time::OffsetDateTime,
    format: Format,
    newline: bool,
    lowercase: bool,
    minecraft: Option<String>,
}

impl Default for Buildstamp {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Buildstamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stamp = self.timestamp.format(self.format.formatter()).unwrap();

        if self.format == Format::Weekly
            && let Some(minecraft) = &self.minecraft {
                stamp.push_str(&minecraft.to_ascii_uppercase());
            }
        if self.lowercase {
            stamp = stamp.to_ascii_lowercase();
        }
        if self.newline {
            stamp.push('\n');
        }

        write!(f, "{stamp}")
    }
}

impl Buildstamp {
    /// Create a new buildstamp with the default format (weekly);
    /// no newline, no lowercase, and no minecraft suffix
    pub fn new() -> Self {
        Self::new_with_format(Format::Weekly)
    }

    /// Create a new buildstamp with the given format;
    /// no newline, no lowercase, and no minecraft suffix
    #[inline]
    pub fn new_with_format(format: Format) -> Self {
        Self::new_from_args(format, false, false, None)
    }

    /// Create a new buildstamp with the given arguments
    #[inline]
    pub fn new_from_args(
        format: Format,
        newline: bool,
        lowercase: bool,
        minecraft: Option<String>,
    ) -> Self {
        Self {
            timestamp: time::OffsetDateTime::now_utc(),
            format,
            newline,
            lowercase,
            minecraft,
        }
    }

    /// Print the buildstamp to stdout
    pub fn print(&self) {
        print!("{self}");
    }
}
