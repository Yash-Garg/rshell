{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    devshell,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [devshell.overlays.default];
      };

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./toolchain.toml;
        sha256 = "sha256-3jVIIf5XPnUU1CRaTyAiO0XHVbJl12MSx3eucTXCjtE=";
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;

      rshell = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
    in {
      checks = {
        inherit rshell;
      };

      packages.default = rshell;

      apps.default = flake-utils.lib.mkApp {
        drv = rshell;
      };

      devShells.default = pkgs.devshell.mkShell {
        env = [
          {
            name = "DEVSHELL_NO_MOTD";
            value = 1;
          }
        ];

        packages = with pkgs; [
          rustStable
        ];
      };
    });
}
