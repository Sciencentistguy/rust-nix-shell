use std::process::{Command, Stdio};
use std::str::FromStr;

use clap::Parser;
use indoc::indoc;
use regex::Regex;

const INVALID_CHANNEL_MSG: &str = "Invalid channel. Channel must be one of:
        - 'stable'
        - 'beta'
        - 'nightly'
        - A specific nightly: 'nightly-YYYY-mm-dd'
        - A rust tagged release, e.g. '1.60' or '1.58.1'";

fn main() {
    let args = Opt::parse();
    let mut cmd = Command::new("nix-shell");

    if args.verbose {
        cmd.arg("--show-trace");
    }

    if args.pure {
        cmd.arg("--pure");
    } else {
        cmd.arg("--command");
        cmd.arg(args.shell);
    }

    let toolchain_attrs = match args.channel {
        RustChannel::Stable => r#"{channel = "stable";}"#.to_owned(),
        RustChannel::Beta => r#"{channel = "beta";}"#.to_owned(),
        RustChannel::Nightly => r#"{channel = "nightly";}"#.to_owned(),
        RustChannel::DatedNightly(date) => format!(r#"{{channel = "nightly"; date = "{date}";}}"#),
        RustChannel::Version(version) => format!(r#"{{channel = "{version}";}}"#),
    };

    let fenix_expr = {
        let fenix_in_nix_path = {
            let mut cmd = Command::new("nix");
            cmd.args([
                "--extra-experimental-features",
                "nix-command flakes",
                "eval",
                "--impure",
                "--expr",
                "<fenix>",
            ]);
            cmd.stdout(Stdio::null());
            let ret = cmd.status().expect("Nix command should not fail");
            ret.code().unwrap() == 0
        };

        if fenix_in_nix_path && !args.fresh_fenix {
            "<fenix>"
        } else {
            r#"fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz""#
        }
    };

    let expression = format!(
        indoc! {"
        ({rust_nix}) {{
            toolchainAttrs = {toolchain_attrs};
            otherDeps = with import <nixpkgs> {{}}; [{extra_packages}];
            fenix = import ({fenix_expr}) {{}};
            name = \"{drv_name}\";
        }}"},
        rust_nix = include_str!("rust.nix"),
        toolchain_attrs = toolchain_attrs,
        extra_packages = args.packages.join(" "),
        drv_name = args.name,
        fenix_expr = fenix_expr,
    );

    if args.verbose {
        eprintln!("Expression to be evaluated:\n{}", expression);
    }

    cmd.arg("-E");
    cmd.arg(expression);
    cmd.status().unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RustChannel {
    Stable,
    Beta,
    Nightly,
    DatedNightly(String),
    Version(String),
}

impl FromStr for RustChannel {
    type Err = &'static str;

    fn from_str(channel: &str) -> Result<Self, Self::Err> {
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
}

/// A nix-based alternative to rustup
#[derive(Parser, Debug)]
struct Opt {
    /// The rust release channel to pull. Possible values: ["stable", "beta", "nightly",
    /// "nightly-YYYY-mm-dd", "1.x.y"]
    #[clap(default_value = "stable")]
    channel: RustChannel,

    /// The shell to open. Passed to 'nix-shell --command'
    #[clap(long, default_value = "zsh")]
    shell: String,

    /// Extra packages to insert in the shell, as if passed to 'nix-shell -p'
    #[clap(short, long, num_args(1..))]
    packages: Vec<String>,

    /// Use a pure nix shell. Overrides '--shell'
    #[clap(long)]
    pure: bool,

    /// Output debug information
    #[clap(long)]
    verbose: bool,

    /// The name for the shell
    #[clap(long, default_value = "rust-nix-shell")]
    name: String,

    /// Force the use of a fresh fenix
    #[clap(long)]
    fresh_fenix: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_nightly() {
        assert_eq!(
            RustChannel::from_str("nightly-2022-01-01"),
            Ok(RustChannel::DatedNightly("2022-01-01".to_string()))
        );
        assert!(RustChannel::from_str("nightly-01/01/2022").is_err());
        assert!(RustChannel::from_str("nightly-2022-1-1").is_err());
    }
    #[test]
    fn test_parsing_version() {
        assert_eq!(
            RustChannel::from_str("1.59"),
            Ok(RustChannel::Version("1.59".to_string()))
        );
        assert_eq!(
            RustChannel::from_str("1.58.1"),
            Ok(RustChannel::Version("1.58.1".to_string()))
        );
        assert!(RustChannel::from_str("1.58.1.5").is_err());
    }

    #[test]
    fn test_parsing_strings() {
        assert_eq!(RustChannel::from_str("stable"), Ok(RustChannel::Stable));
        assert_eq!(RustChannel::from_str("beta"), Ok(RustChannel::Beta));
        assert_eq!(RustChannel::from_str("nightly"), Ok(RustChannel::Nightly));
        assert!(RustChannel::from_str("nightly-").is_err());
        assert!(RustChannel::from_str("something else").is_err());
    }
}
