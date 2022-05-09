{
  inputs = {
    # github example, also supported gitlab:
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
        generic-rust-shell = self.packages.${prev.system}.default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        generic-rust-shell = {
          lib,
          rustPlatform,
        }:
          rustPlatform.buildRustPackage {
            name = "generic-rust-shell";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = with lib; {
              license = licenses.mpl20;
              homepage = "https://github.com/Sciencentistguy/generic-rust-shell";
              platforms = platforms.all;
            };
          };
      in {
        packages.generic-rust-shell = pkgs.callPackage generic-rust-shell {};
        packages.default = self.packages.${system}.generic-rust-shell;

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
