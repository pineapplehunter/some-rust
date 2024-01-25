{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
    in
    {
      packages.x86_64-linux.default = pkgs.callPackage ./package.nix { };

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs;[ pkgsCross.riscv64.stdenv.cc pkgsCross.riscv64.binutils ];
      };

      legacyPackages.x86_64-linux = pkgs;

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
