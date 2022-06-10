use std::process::{Command, Stdio};

use clap::Parser;
use indoc::indoc;

mod cli;

fn main() {
    let args = cli::Opt::parse();
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

    let toolchain_attrs = {
        use cli::RustChannel;
        match args.channel {
            RustChannel::Stable => r#"{channel = "stable";}"#.to_owned(),
            RustChannel::Beta => r#"{channel = "beta";}"#.to_owned(),
            RustChannel::Nightly => r#"{channel = "nightly";}"#.to_owned(),
            RustChannel::DatedNightly(date) => {
                format!(r#"{{channel = "nightly"; date = "{date}";}}"#)
            }
            RustChannel::Version(version) => format!(r#"{{channel = "{version}";}}"#),
        }
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

        match fenix_in_nix_path {
            true if !args.fresh_fenix => "<fenix>",
            _ => r#"fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz""#,
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

