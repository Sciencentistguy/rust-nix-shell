# rust-nix-shell

Have you ever found yourself wanting to work on a rust project on NixOS without having to write a `flake.nix` or a `shell.nix` for that project? You need `rust-nix-shell`, a program halfway between `rustup` and `nix-shell`! No more are the glibc errors that plague `rustup` on NixOS; gone are the horrible `nix-shell -E '...'` incantations to set `RUST_SRC_PATH` properly in an ephemeral shell.

`rust-nix-shell` allows you to create a shell environment with any version of rust, à la `rustup`. You can also specify other dependencies with `-p`, à la `nix-shell`.

## Dependencies

This program shells out to `nix-shell`, finding it from `$PATH`. It also by default calls `zsh` from `$PATH`, but this is configurable

## Usage

`rust-nix-shell --help`

You can specify the channel as an optional argument. This must be one of:

- `stable`
- `beta`
- `nightly`
- A specific nightly: `nightly-YYYY-mm-dd`
- A rust tagged release, e.g. `1.60` or `1.58.1`

---

Available under the terms of version 2.0 of the Mozilla Public Licence
