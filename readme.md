# rust-nix-shell

Have you ever found yourself wanting to work on a rust project on NixOS without having to write a `flake.nix` or a `shell.nix` for that project? You need `rust-nix-shell`, a program halfway between `rustup` and `nix-shell`! No more are the glibc errors that plague `rustup` on NixOS; gone are the horrible `nix-shell -E '...'` incantations to set `RUST_SRC_PATH` properly in an ephemeral shell.

`rust-nix-shell` allows you to create a shell environment with any version of rust, à la `rustup`. You can also specify other dependencies with `-p`, à la `nix-shell`.

## Note

This program is intended for use on NixOS. If your system is not NixOS then I'd recommend using `rustup` instead of this --- unless you **know** that you want a nix-based rust environment, you probably don't.

It produces a shell with a complete rust toolchain of the specified version (the most recent stable, by default). This is useful when you're working on a project that does not have nix development files in the repo, or if they are not compatible with a nix-based rust-analyzer installation.

## Dependencies

This program shells out to `nix-shell`, finding it from `$PATH`. It also by default calls `zsh` from `$PATH`, but this is configurable

Optionally, if a copy of [fenix](https://github.com/nix-community/fenix) is found at `<fenix>` (in `$NIX_PATH`), it will be used instead of downloading fenix master using `fetchtarball` every time.

## Installation

This repository is a flake, providing the output `rust-nix-shell`, and contains a `default.nix` for backwards compatibility.

The recommended way of installing this is by adding it to your system configuration. (For an example, look at my [system flake](https://github.com/Sciencentistguy/nixfiles))

Failing that, it can be run ephemerally with `nix run`:

```sh
nix run 'github:Sciencentistguy/rust-nix-shell' -- [rust-nix-shell arguments]
```

Or, it can be installed using `nix profile`:

```sh
nix profile install 'github:Sciencentistguy/rust-nix-shell'
```

For backwards compatibility, a `default.nix` is provided, allowing `rust-nix-shell` to be installed using `nix-env`:

```sh
nix-env --dry-run -f 'https://github.com/Sciencentistguy/rust-nix-shell/archive/master.tar.gz' -i rust-nix-shell
```

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
