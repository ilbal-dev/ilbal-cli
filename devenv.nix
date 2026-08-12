{ pkgs, lib, ... }:

{
  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      figlet
      lolcat
      # Required for cargo-tarpaulin
      openssl
      pkg-config
      # rustls is a pure rust option to explore instead of the above
      # cargo-cross
    ]
    ++ lib.optionals pkgs.stdenv.isLinux [
      pkgsCross.mingwW64.stdenv.cc
    ];

  env = {
    RUST_BACKTRACE = "1";
    CARGO_TERM_COLOR = "always";
  }
  // lib.optionalAttrs pkgs.stdenv.isLinux {
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
    CC_x86_64_pc_windows_gnu = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
    AR_x86_64_pc_windows_gnu = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-ar";
  };

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "rust-src"
      "llvm-tools"
    ];
    targets = [
      "x86_64-pc-windows-gnu"
      "aarch64-apple-darwin"
      "x86_64-apple-darwin"
    ];
  };

  # https://devenv.sh/scripts/
  scripts = {
    init.exec = ''
      figlet Rust dev | lolcat
      echo ""
      echo "🦀 Toolchain:"
      rustc --version
      cargo --version
      for tool in cargo-watch cargo-expand cargo-nextest cargo-tarpaulin cargo-audit cargo-insta; do
        command -v "$tool" >/dev/null || echo "⚠️  $tool not found — run: cargo install --locked $tool"
      done
    '';
  };

  # https://devenv.sh/basics/
  enterShell = ''
    export CC="${pkgs.stdenv.cc}/bin/cc"
    export CXX="${pkgs.stdenv.cc}/bin/c++"
    export AR="${pkgs.stdenv.cc}/bin/ar"
    export RANLIB="${pkgs.stdenv.cc}/bin/ranlib"

    init
  '';
}
