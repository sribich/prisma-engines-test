{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    binaryen
    cargo-insta
    cargo-nextest
    cargo-watch
    git
    graphviz
    jq
    llvmPackages_latest.bintools
    rustup
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl.dev
  ];
}
