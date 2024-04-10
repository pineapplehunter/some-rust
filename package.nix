{ lib
, stdenvNoCC
, rust-bin
, makeRustPlatform
, pkgsCross
, cacert
}:

let
  riscv64 = pkgsCross.riscv64-embedded;
  latest-nightly = rust-bin.nightly.latest;
  rust-bin-nightly = latest-nightly.default.override {
    extensions = [ "rust-src" ];
  };
  rustPlatform = (makeRustPlatform {
    rustc = rust-bin-nightly;
    cargo = rust-bin-nightly;
  });
in
stdenvNoCC.mkDerivation rec {
  pname = "rust-test-programs";
  version = "0.1.0";

  src = lib.sources.sourceByRegex ./. [
    "Cargo.*"
    "src.*"
    "riscv64-custom.json"
    "Makefile"
    ".cargo.*"
    "linker.ld"
    "build.rs"
  ];

  nativeBuildInputs = [ rust-bin-nightly rustPlatform.cargoSetupHook riscv64.stdenv.cc ];

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-7VpvhxLgSqcH25Y0kkkxUCEIQ94PnEe7nZrj+yzIkc0=";

    buildPhase = ''
      runHook preBuild

      # Ensure deterministic Cargo vendor builds
      export SOURCE_DATE_EPOCH=1

      if [[ ! -f Cargo.lock ]]; then
          echo
          echo "ERROR: The Cargo.lock file doesn't exist"
          echo
          echo "Cargo.lock is needed to make sure that cargoHash/cargoSha256 doesn't change"
          echo "when the registry is updated."
          echo

          exit 1
      fi

      # Keep the original around for copyLockfile
      cp Cargo.lock Cargo.lock.orig

      export CARGO_HOME=$(mktemp -d cargo-home.XXX)
      CARGO_CONFIG=$(mktemp cargo-config.XXXX)

      if [[ -n "$NIX_CRATES_INDEX" ]]; then
      cat >$CARGO_HOME/config.toml <<EOF
      [source.crates-io]
      replace-with = 'mirror'
      [source.mirror]
      registry = "$NIX_CRATES_INDEX"
      EOF
      fi

      # Override the `http.cainfo` option usually specified in `.cargo/config`.
      export CARGO_HTTP_CAINFO=${cacert}/etc/ssl/certs/ca-bundle.crt

      if grep '^source = "git' Cargo.lock; then
          echo
          echo "ERROR: The Cargo.lock contains git dependencies"
          echo
          echo "This is currently not supported in the fixed-output derivation fetcher."
          echo "Use cargoLock.lockFile / importCargoLock instead."
          echo

          exit 1
      fi

      cp -rL --no-preserve=mode ${latest-nightly.rust-src}/lib/rustlib/src/rust rust-src
      cp rust-src/Cargo.lock rust-src/library/test/

      cargo vendor $name \
        --respect-source-config \
        --sync rust-src/library/test/Cargo.toml \
         | cargo-vendor-normalise > $CARGO_CONFIG

      # Create an empty vendor directory when there is no dependency to vendor
      mkdir -p $name
      # Add the Cargo.lock to allow hash invalidation
      cp Cargo.lock.orig $name/Cargo.lock

      # Packages with git dependencies generate non-default cargo configs, so
      # always install it rather than trying to write a standard default template.
      install -D $CARGO_CONFIG $name/.cargo/config;

      runHook postBuild
    '';
  };

  makeFlags = [
    "RISCV_PREFIX=${riscv64.stdenv.cc.targetPrefix}"
  ];

  installPhase = ''
    mkdir $out
    cp output/* $out/
  '';
}
