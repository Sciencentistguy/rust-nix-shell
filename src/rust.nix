{
  toolchainAttrs,
  otherDeps,
}: let
  rust-nix-shell = {
    mkShell,
    toolchain,
    cargo-edit,
    pkg-config,
    rustPlatform,
  }:
    mkShell {
      name = "rust-nix-shell";
      buildInputs =
        [
          (toolchain.withComponents ["cargo" "rustc" "rust-src" "rustfmt" "clippy"])
          cargo-edit

          pkg-config
          rustPlatform.bindgenHook
        ]
        ++ otherDeps;

      RUST_SRC_PATH = "${toolchain.rust-src}";
    };
  pkgs = import <nixpkgs> {};
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  toolchain = fenix.toolchainOf toolchainAttrs;
in
  pkgs.callPackage rust-nix-shell {
    inherit toolchain;
    rustPlatform = pkgs.makeRustPlatform {
      inherit (toolchain) rustc cargo;
    };
  }
