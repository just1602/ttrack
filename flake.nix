{
  description = "A simple cli time tracker";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = pkgs.callPackage nix/package.nix { };
        };

        legacyPackages = {
          default = pkgs.callPackage nix/package.nix { };
        };

        devShells = {
          default = pkgs.mkShell {
            packages = [
              pkgs.rustc
              pkgs.cargo
              pkgs.rust-analyzer
            ];
          };
        };

      }
    )
    // {
      overlays.default = final: prev: {
        ttrack = prev.pkgs.callPackage nix/package.nix { };
      };
    };
}
