{ pkgs ? import <nixpkgs> {}, }:

with pkgs;

mkShell {
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [
    (pkgs.fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "llvm-tools-preview"
      "rustc-codegen-cranelift-preview"
    ])
    rust-analyzer-nightly
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
}