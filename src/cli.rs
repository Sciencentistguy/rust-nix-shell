use clap::Parser;
use regex::Regex;

const INVALID_CHANNEL_MSG: &str = "Invalid channel. Channel must be one of:
        - 'stable'
        - 'beta'
        - 'nightly'
        - A specific nightly: 'nightly-YYYY-mm-dd'
        - A rust tagged release, e.g. '1.60' or '1.58.1'";

/// A nix-based alternative to rustup
#[derive(Parser, Debug)]
pub struct Opt {
    /// The rust release channel to pull. Possible values: ["stable", "beta", "nightly",
    /// "nightly-YYYY-mm-dd", "1.x.y"]
    #[clap(default_value = "stable", parse(try_from_str = parse_channel_str))]
    pub channel: RustChannel,

    /// The shell to open. Passed to 'nix-shell --command'
    #[clap(long, default_value = "zsh")]
    pub shell: String,

    /// Extra packages to insert in the shell, as if passed to 'nix-shell -p'
    #[clap(short, long, multiple_values(true))]
    pub packages: Vec<String>,

    /// Use a pure nix shell. Overrides '--shell'
    #[clap(long)]
    pub pure: bool,

    /// Output debug information
    #[clap(long)]
    pub verbose: bool,

    /// The name for the shell
    #[clap(long, default_value = "rust-nix-shell")]
    pub name: String,

    /// Force the use of a fresh fenix
    #[clap(long)]
    pub fresh_fenix: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RustChannel {
    Stable,
    Beta,
    Nightly,
    DatedNightly(String),
    Version(String),
}

pub fn parse_channel_str(channel: &str) -> Result<RustChannel, &'static str> {
    let nightly_pattern = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}$").unwrap();
    let version_pattern = Regex::new(r"^1\.\d+(?:\.\d+)?$").unwrap();
    match channel {
        "stable" => Ok(RustChannel::Stable),
        "beta" => Ok(RustChannel::Beta),
        "nightly" => Ok(RustChannel::Nightly),
        _ if nightly_pattern.is_match(channel) => {
            Ok(RustChannel::DatedNightly(channel[8..].to_owned()))
        }
        _ if version_pattern.is_match(channel) => Ok(RustChannel::Version(channel.to_owned())),
        _ => Err(INVALID_CHANNEL_MSG),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_nightly() {
        assert_eq!(
            parse_channel_str("nightly-2022-01-01"),
            Ok(RustChannel::DatedNightly("2022-01-01".to_string()))
        );
        assert!(parse_channel_str("nightly-01/01/2022").is_err());
        assert!(parse_channel_str("nightly-2022-1-1").is_err());
    }
    #[test]
    fn test_parsing_version() {
        assert_eq!(
            parse_channel_str("1.59"),
            Ok(RustChannel::Version("1.59".to_string()))
        );
        assert_eq!(
            parse_channel_str("1.58.1"),
            Ok(RustChannel::Version("1.58.1".to_string()))
        );
        assert!(parse_channel_str("1.58.1.5").is_err());
    }

    #[test]
    fn test_parsing_strings() {
        assert_eq!(parse_channel_str("stable"), Ok(RustChannel::Stable));
        assert_eq!(parse_channel_str("beta"), Ok(RustChannel::Beta));
        assert_eq!(parse_channel_str("nightly"), Ok(RustChannel::Nightly));
        assert!(parse_channel_str("nightly-").is_err());
        assert!(parse_channel_str("something else").is_err());
    }
}
