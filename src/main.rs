use std::process::{self, Command};

use clap::Parser;
use regex::Regex;

fn main() {
    let args = Opt::parse();

    let channel = &args.channel;
    let channel = match parse_channel_str(channel) {
        Some(x) => x,
        None => {
            eprintln!("Invalid version specified: {channel}");
            process::exit(1);
        }
    };
    open_shell(channel, &args.shell);
}

fn open_shell(channel: RustChannel, shell: &str) {
    let mut cmd = Command::new("nix-shell");
    cmd.arg("--show-trace");
    cmd.arg("--command");
    cmd.arg(shell);
    cmd.arg("-E");

    let expression = format!(
        "({}) {}",
        include_str!("generic-rust.nix"),
        match channel {
            RustChannel::Stable => r#"{channel = "stable";}"#.to_owned(),
            RustChannel::Beta => r#"{channel = "beta";}"#.to_owned(),
            RustChannel::Nightly => r#"{channel = "nightly";}"#.to_owned(),
            RustChannel::DatedNightly(date) =>
                format!(r#"{{channel = "nightly"; date = "{date}";}}"#),
            RustChannel::Version(version) => format!(r#"{{channel = "{version}";}}"#),
        }
    );

    cmd.arg(expression);
    cmd.status().unwrap();
}

#[derive(Debug, PartialEq, Eq)]
enum RustChannel {
    Stable,
    Beta,
    Nightly,
    DatedNightly(String),
    Version(String),
}

fn parse_channel_str(channel: &str) -> Option<RustChannel> {
    match channel {
        "stable" => Some(RustChannel::Stable),
        "beta" => Some(RustChannel::Beta),
        "nightly" => Some(RustChannel::Nightly),
        _ => {
            let nightly_pattern = Regex::new(r"^nightly-\d+-\d+-\d+$").unwrap();
            let version_pattern = Regex::new(r"^1\.\d+(?:\.\d+)?$").unwrap();

            if nightly_pattern.is_match(channel) {
                Some(RustChannel::DatedNightly(channel[8..].to_owned()))
            } else if version_pattern.is_match(channel) {
                Some(RustChannel::Version(channel.to_owned()))
            } else {
                None
            }
        }
    }
}

/// A nix-based alternative to `rustup`
#[derive(Parser, Debug)]
struct Opt {
    /// The rust release channel to pull. Possible values: ["stable", "beta", "nightly",
    /// "nightly-YYYY-mm-dd", "1.x.y"]
    #[clap(default_value = "stable")]
    channel: String,

    /// The shell to open. Passed to `nix-shell --command`
    #[clap(long, default_value = "zsh")]
    shell: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_nightly() {
        assert_eq!(
            parse_channel_str("nightly-2022-01-01").unwrap(),
            RustChannel::DatedNightly("2022-01-01".to_string())
        );
        assert_eq!(parse_channel_str("nightly-01/01/2022"), None);
    }
    #[test]
    fn test_parsing_version() {
        assert_eq!(
            parse_channel_str("1.59").unwrap(),
            RustChannel::Version("1.59".to_string())
        );
        assert_eq!(
            parse_channel_str("1.58.1").unwrap(),
            RustChannel::Version("1.58.1".to_string())
        );
        assert_eq!(parse_channel_str("1.58.1.5"), None);
    }
}
