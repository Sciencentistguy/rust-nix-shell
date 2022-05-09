# generic-rust-shell

Enter a nix-shell environment that has all the tools needed to develop rust code. Defaults to stable, but can also use other channels.

## Usage

Make sure you have `nix-shell` on your `PATH`.

You can specify the channel as an optional argument. This must be one of:

- `stable`
- `beta`
- `nightly`
- A specific nightly: `nightly-YYYY-mm-dd`
- A rust tagged release, e.g. `1.60` or `1.58.1`

You can also speciy the shell (defaults to `zsh`, also from `PATH`)

--- 

Available under the terms of version 2.0 of the Mozilla Public Licence
