{
  pkgs,
  self,
  system,
}:
{
  default = pkgs.mkShell (
    let
      rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (
        toolchain:
        toolchain.default.override {
          extensions = [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
            "llvm-tools-preview"
            "rustc-codegen-cranelift-preview"
            "miri"
          ];
          # targets = [ "arm-unknown-linux-gnueabihf" ];
        }
      );
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

    in
    {
      NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";

      packages = [
        rustToolchain
      ]
      ++ (with pkgs; [
        # rust-analyzer-nightly
        cargo-binstall
        cargo-llvm-cov
        cargo-nextest
        cargo-mutants
        cargo-machete

        cargo-watch
        bacon

        # cargo-audit
        # cargo-deny
        # cargo-geiger
        # cargo-outdated
        # cargo-insta
        # cargo-hack
        # grcov
        # bunyan-rs
        # valgrind
        # cargo-valgrind

        mecab
        curl
        wget
        pkg-config
        openssl

        lld
        binutils
        mold-wrapped
        clang
        libclang
        cmake
      ]);

      buildInputs = with pkgs; [
        curl
        wget
        pkg-config
        openssl
      ];

      shellHook =
        let
          libraries = with pkgs; [
            openssl
            librsvg
            binutils
            mpv

            # Needed to run cargo-interactive-update
            curl

            cmake
            clang
            libclang
          ];
        in
        ''
          export LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
          export EXTRA_CCFLAGS="-I/usr/include"
        '';
    }
  );
}
