{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "ducat-dev";

  buildInputs = with pkgs; [
    rustup
    pkg-config
    openssl
    cmake
    curl
    git
    protobuf
    libclang
    llvm
    libiconv
    libsodium
    clang
  ];

  nativeBuildInputs = with pkgs; [
    which
  ];

  shellHook = ''
    export RUSTUP_HOME=$HOME/.rustup
    export CARGO_HOME=$HOME/.cargo
    export PATH=$CARGO_HOME/bin:$PATH

    echo "Welcome to the nix-shell dev env"

#    RUST_LOG=debug cargo run
    export RUST_LOG=debug
  '';
}
