{ lib
, config
, dream2nix
, ...
}: {
  imports = [
    # dream2nix.modules.dream2nix.rust-cargo-lock
    # dream2nix.modules.dream2nix.buildRustPackage
    dream2nix.modules.dream2nix.mkDerivation
  ];

  deps = { nixpkgs, ... }: {
    riscv64 = nixpkgs.pkgsCross.riscv64-embedded;
    latest-nightly = nixpkgs.rust-bin.nightly.latest;
    rust-bin = config.deps.latest-nightly.default.override {
      extensions = [ "rust-src" ];
    };
    riscv-stdenv-cc = config.deps.riscv64.stdenv.cc;
    rustPlatform = (nixpkgs.makeRustPlatform {
      rustc = config.deps.rust-bin;
      cargo = config.deps.rust-bin;
    });
  };

  name = lib.mkForce "rust-benches";
  version = lib.mkForce "0.1.0";

  mkDerivation = {
    src = lib.cleanSource ./.;
    nativeBuildInputs = with config.deps;[ rust-bin ];
  };
}
