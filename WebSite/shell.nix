{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    trunk
    wasm-bindgen-cli
    gcc
    llvmPackages.lld
  ];
}
