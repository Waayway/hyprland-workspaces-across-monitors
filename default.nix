{pkgs ? import <nixpkgs> {}, ...}: 

pkgs.stdenv.mkDerivation {
  name = "workspace-across-monitors";
  src = ./.;

  nativeBuildInputs = with pkgs; [
    rustc
    cargo
  ];

  buildPhase = ''
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/wam $out/bin
  '';
}