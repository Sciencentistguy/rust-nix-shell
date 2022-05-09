# generic-rust-shell

Enter a nix-shell environment that has all the tools needed to develop rust code. Defaults to stable, but can also use other channels.

Intended as a nix-based alternative to `rustup`. Rustup on nixos has issues with glibc; this does not.

## Dependencies

This program shells out to `nix-shell`, finding it from `$PATH`. It also by default calls `zsh` from `$PATH`, but this is configurable

## Usage

`generic-rust-shell --help`

You can specify the channel as an optional argument. This must be one of:
- `stable`
- `beta`
- `nightly`
- A specific nightly: `nightly-YYYY-mm-dd`
- A rust tagged release, e.g. `1.60` or `1.58.1`

---

Available under the terms of version 2.0 of the Mozilla Public Licence
