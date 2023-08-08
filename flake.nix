{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      pkgsCross = import nixpkgs {
        localSystem = "x86_64-linux";
        crossSystem = nixpkgs.legacyPackages.x86_64-linux.lib.systems.examples.riscv64-embedded // {
          rustc.config = "riscv64-custom";
          rustc.platform = builtins.fromJSON (builtins.readFile ./riscv64-custom.json);
        };
      };
      rust-bin = pkgs.rust-bin.nightly.latest.minimal.override {
        extensions = [ "rust-src" ];
      };
    in
    {

      packages.x86_64-linux.default = (pkgs.makeRustPlatform {
        rustc = rust-bin;
        cargo = rust-bin;
      }).buildRustPackage {
        pname = "ha";
        version = "819203";

        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        doCheck = false;
        buildPhase = ''
          make OBJDUMP="${pkgs.pkgsCross.riscv64.binutils}/bin/objdump" OBJCOPY="${pkgs.pkgsCross.riscv64.binutils}/bin/objcopy"
        '';

        installPhase = ''
          mkdir $out
          cp output/* $out/
        '';
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;

    };
}
