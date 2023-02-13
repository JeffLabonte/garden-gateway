let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust = (
    nixpkgs.rustChannelOf {
      channel = "stable";
    }).rust.override {
    extensions = [
      "rust-src"
      "rust-analysis"
      "rustfmt-preview"
      "clippy-preview"
    ];
  };
in
with nixpkgs;
stdenv.mkDerivation {
  name = "Garden Gateway";
  buildInputs = [
    rust
    rust-analyzer
    cargo-tarpaulin
    pkgconfig
    openssl
    sqlite
    rustc
    gnumake
    clippy
    rustfmt
    lldb
  ];
  packages = with nixpkgs; [
    clippy
    cargo
    git
    git-lfs
  ];
  # Set Environment Variables
  RUST_BACKTRACE = 1;

}
