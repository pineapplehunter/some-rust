{
  lib,
  stdenvNoCC,
  rust-bin,
  makeRustPlatform,
  symlinkJoin,
  pkgsCross,
}:

let
  rust-bin-nightly = rust-bin.selectLatestNightlyWith (
    toolchain:
    toolchain.minimal.override {
      extensions = [ "rust-src" ];
    }
  );
  rustPlatform = (
    makeRustPlatform {
      rustc = rust-bin-nightly;
      cargo = rust-bin-nightly;
    }
  );
  riscv64 = pkgsCross.riscv64-embedded;
in
stdenvNoCC.mkDerivation rec {
  pname = "rust-test-programs";
  version = "0.1.0";

  enablePaarallelBuilding = true;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./.cargo
      ./Cargo.lock
      ./Cargo.toml
      ./Makefile
      ./build.rs
      ./linker.ld
      ./riscv64-custom.json
      ./src
    ];
  };

  nativeBuildInputs = [
    rust-bin-nightly
    rustPlatform.cargoSetupHook
    riscv64.stdenv.cc
  ];

  cargoDeps = symlinkJoin {
    name = "cargo-vendor-with-std";
    paths = [
      # order is important !!!
      (rustPlatform.fetchCargoVendor {
        inherit src pname version;
        hash = "sha256-euRgvw4g8qweZyZbT5/AyFUXNZ/S7bNE7Hr7B3IOIQw=";
      })
      (rustPlatform.fetchCargoVendor {
        name = "rust-std";
        src = "${rust-bin-nightly}/lib/rustlib/src/rust/library";
        hash = "sha256-0fkHVqtvZbVm2fhVB+8IomaWKMMl3N4NvOe6qYp7304=";
      })
    ];
  };

  makeFlags = [
    "RISCV_PREFIX=${riscv64.stdenv.cc.targetPrefix}"
  ];

  installPhase = ''
    mkdir $out
    cp output/* $out/
  '';
}
