{
  lib,
  stdenv,
  rust-bin,
  rustPlatform,
  lndir,
  pkgsCross,
}:

let
  rust = rust-bin.selectLatestNightlyWith (
    toolchain:
    toolchain.minimal.override {
      extensions = [ "rust-src" ];
    }
  );

  riscv64 = pkgsCross.riscv64-embedded;
in
stdenv.mkDerivation (finalAttrs: {
  pname = "rust-test-programs";
  version = "0.1.0";

  enableParallelBuilding = true;

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

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    rust
    lndir
    rustPlatform.cargoSetupHook
    riscv64.stdenv.cc
  ];

  preBuild = ''
    lndir "${rust}/lib/rustlib/src/rust/library/vendor" /build/cargo-vendor-dir
  '';

  makeFlags = [
    "RISCV_PREFIX=${riscv64.stdenv.cc.targetPrefix}"
  ];

  installPhase = ''
    runHook preInstall
    mkdir -p "$out"
    cp output/* "$out"/
    runHook postInstall
  '';
})
