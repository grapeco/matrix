{ pkgs ? import <nixpkgs> {}, }:

with pkgs;

mkShell {
  buildInputs = with pkgs; [
    (pkgs.fenix.complete.withComponents [
      "cargo"
      "rustc"
      "rust-src"
    ])
    rust-analyzer-nightly
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
}
