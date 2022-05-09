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
        generic-nix-shell = self.packages.${prev.system}.default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        generic-nix-shell = {
          lib,
          rustPlatform,
        }:
          rustPlatform.buildRustPackage {
            name = "generic-nix-shell";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = with lib; {
              license = licenses.mpl20;
              homepage = "https://github.com/Sciencentistguy/generic-nix-shell";
              platforms = platforms.all;
            };
          };
      in {
        packages.default = pkgs.callPackage generic-nix-shell {};
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
