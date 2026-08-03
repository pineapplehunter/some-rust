{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        { pkgs, system, ... }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };
          packages.default = pkgs.callPackage ./package.nix { };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [
                    "rust-analyzer"
                    "rust-src"
                  ];
                }
              ))
              gnumake
              pkgsCross.riscv64-embedded.stdenv.cc
            ];
          };

          formatter = pkgs.nixfmt-tree;
        };
    };
}
