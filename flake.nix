{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    dream2nix = {
      url = "github:nix-community/dream2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, dream2nix }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
    in
    {
      packages.x86_64-linux.default = pkgs.callPackage ./package.nix { };

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs;[ gnumake pkgsCross.riscv64-embedded.stdenv.cc ];
      };

      legacyPackages.x86_64-linux = pkgs;

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
