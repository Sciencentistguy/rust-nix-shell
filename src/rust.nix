{
  fenix,
  name,
  otherDeps,
  toolchainAttrs,
}: let
  rust-nix-shell = {
    cargo-edit,
    lib,
    mkShell,
    pkg-config,
    rustPlatform,
    toolchain,
  }:
    mkShell {
      inherit name;
      buildInputs =
        [
          (toolchain.withComponents ["cargo" "rustc" "rust-src" "rustfmt" "clippy"])
          cargo-edit

          pkg-config
          rustPlatform.bindgenHook
        ]
        ++ otherDeps;

      RUST_SRC_PATH = "${toolchain.rust-src}";
      LD_LIBRARY_PATH = lib.makeLibraryPath otherDeps;
    };
  pkgs = import <nixpkgs> {};
  inherit fenix;
  toolchain = fenix.toolchainOf toolchainAttrs;
in
  pkgs.callPackage rust-nix-shell {
    inherit toolchain;
    rustPlatform = pkgs.makeRustPlatform {
      inherit (toolchain) cargo;
      rustc = toolchain.rustc // {inherit (pkgs) llvmPackages;};
    };
  }
