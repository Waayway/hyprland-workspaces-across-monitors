{pkgs ? import <nixpkgs> {}, ...}: 

pkgs.rustPlatform.buildRustPackage rec {
  pname = "wam";
  version = "0.0.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}


# pkgs.stdenv.mkDerivation {
#   name = "workspace-across-monitors";
#   src = ./.;

#   nativeBuildInputs = with pkgs; [
#     rustup
#   ];

#   buildPhase = ''
#     rustup default stable
#     cargo build --release
#   '';

#   installPhase = ''
#     mkdir -p $out/bin
#     cp target/release/wam $out/bin
#   '';
# }