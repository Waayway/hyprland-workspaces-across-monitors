{
  description = "Hyprland Workspaces across multiple monitors";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        wam = pkgs.rustPlatform.buildRustPackage
          {
            pname = "workspace-across-monitors";
            version = "0.0.1";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = [ pkgs.pkg-config ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
      in
      {
        packages = rec {
          wam = wam;
          defaultPackage.${system} = wam;
        };
        apps = rec {
          wam = flake-utils.lib.mkApp { drv = self.packages.${system}.wam; };
          default = wam;
        };
        devShell = pkgs.mkShell {
          buildInputs = [
            wam
          ];
        };
      }
    );
}
