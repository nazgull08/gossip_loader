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

    # === Logging presets ===
    export RUST_LOG_DEFAULT="info,ws_server=debug,node=debug,actors=debug"
    export RUST_LOG_VERBOSE="debug,ws_server=debug,node=debug,actors=debug,libp2p=debug,libp2p_gossipsub=trace,libp2p_swarm=debug,libp2p_kad=debug,libp2p_tcp=debug"
    export RUST_LOG_MINIMAL="warn,libp2p=warn,libp2p_kad=warn,libp2p_swarm=warn,libp2p_tcp=warn"

    alias log-default='export RUST_LOG="$RUST_LOG_DEFAULT" && echo "RUST_LOG set to default"'
    alias log-verbose='export RUST_LOG="$RUST_LOG_VERBOSE" && echo "RUST_LOG set to verbose"'
    alias log-minimal='export RUST_LOG="$RUST_LOG_MINIMAL" && echo "RUST_LOG set to minimal"'

    # By default, use minimal logs
    log-verbose
  '';
}
