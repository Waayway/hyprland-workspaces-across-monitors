{
  description = "Hyprland Workspaces across multiple monitors";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    # flake-utils.lib.eachSystem [ system.x86_64-linux ]
    #   (system:
        let
          system = "x86_64-linux";
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          packages.${system}.default = pkgs.callPackage ./default.nix { inherit pkgs; };
        };
      # );
}
