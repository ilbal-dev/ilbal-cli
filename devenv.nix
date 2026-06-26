{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    figlet
    lolcat
    # rustls is a pure rust option instead of the below
    # openssl
    # pkg-config
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

  tasks = {
    "rust:cargo-setup" = {
      exec = ''
        for tool in cargo-watch cargo-expand cargo-nextest cargo-tarpaulin cargo-audit cargo-insta; do
          command -v "$tool" >/dev/null || echo "⚠️  $tool not found — run: cargo install --locked $tool"
        done
      '';
      after = [ "devenv:enterShell" ];
    };
  };

  # https://devenv.sh/scripts/
  scripts = {
    info.exec = ''
      figlet Rust dev | lolcat
      echo ""
      echo "🦀 Toolchain:"
      rustc --version
      cargo --version
    '';
  };

  # https://devenv.sh/basics/
  enterShell = ''
    info
  '';
}
