{
  description = "A simple cli time tracker";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
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

        devShells = {
          default = pkgs.mkShell {
            packages = [
              pkgs.rustc
              pkgs.cargo
            ];
          };
        };
      }
    );
}
