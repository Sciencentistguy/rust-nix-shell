{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    {
      overlay = final: prev: {
        rust-nix-shell = self.packages.${prev.system}.default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        rust-nix-shell = {
          lib,
          rustPlatform,
        }:
          rustPlatform.buildRustPackage {
            name = "rust-nix-shell";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = with lib; {
              description = "A nix-based alternative to rustup";
              license = licenses.mpl20;
              homepage = "https://github.com/Sciencentistguy/rust-nix-shell";
              platforms = platforms.all;
            };
          };
      in {
        packages.rust-nix-shell = pkgs.callPackage rust-nix-shell {};
        packages.default = self.packages.${system}.rust-nix-shell;

        devShells.default = self.packages.${system}.default.overrideAttrs (super: {
          nativeBuildInputs = with pkgs;
            super.nativeBuildInputs
            ++ [
              cargo-edit
              clippy
              rustfmt
            ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        });
      }
    );
}
