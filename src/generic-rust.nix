{toolchainAttrs, otherDeps}: let
  generic-rust-shell = {
    cargo,
    cargo-edit,
    clippy,
    mkShell,
    openssl,
    pkg-config,
    rust-src,
    rustPlatform,
    rustc,
    rustfmt,
  }:
    mkShell {
      name = "generic-rust-shell";
      buildInputs = [
        rustc
        cargo
        rustfmt
        clippy
        cargo-edit

        pkg-config
        rustPlatform.bindgenHook
      ] ++ otherDeps;

      RUST_SRC_PATH = "${rust-src}";
    };
  pkgs = import <nixpkgs> {};
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  toolchain = fenix.toolchainOf toolchainAttrs;
in
  pkgs.callPackage generic-rust-shell {
    inherit (toolchain) cargo clippy rustc rustfmt rust-src;
    rustPlatform = pkgs.makeRustPlatform {
      inherit (toolchain) rustc cargo;
    };
  }
