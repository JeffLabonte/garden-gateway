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
      rustup
        rust
        rust-analyzer
        pkgconfig 
        openssl 
        sqlite
        rustc
        gnumake
        git
        git-lfs
        clippy
        rustfmt
     ];
    packages = with nixpkgs; [
      clippy
      python310
      cargo
      ripgrep
      nerdfonts
    ];
    # Set Environment Variables
    RUST_BACKTRACE = 1;
  }
