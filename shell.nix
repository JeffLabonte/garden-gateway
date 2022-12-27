let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "Garden Gateway";
    buildInputs = [
        latest.rustChannels.nightly.rust
        pkgconfig 
        openssl 
        sqlite
        cargo
        rustc
        gnumake
        git
        git-lfs
     ];
    packages = with pkgs; [
       starship
       zsh
       neovim
       python310
    ];
    # Set Environment Variables
    RUST_BACKTRACE = 1;
    shellHook = ''
      zsh
      make
      make copy_env_template

      starship init zsh
 k   '';
  }
