{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    figlet
    lolcat
    # Required for cargo-tarpaulin
    openssl
    pkg-config
    # rustls is a pure rust option to explore instead of the above
  ];

  env = {
    RUST_BACKTRACE = "1";
    CARGO_TERM_COLOR = "always";
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
    init
  '';
}
