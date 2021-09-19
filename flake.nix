{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        # `nix build`
        packages.january = naersk-lib.buildPackage {
          nativeBuildInputs = with pkgs; [ pkg-config rustc cargo openssl ];
          pname = "january";
          root = ./.;
        };
        defaultPackage = packages.january;

        # `nix run`
        apps.january = flake-utils.lib.mkApp {
          drv = packages.january;
        };

        defaultApp = apps.january;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ pkg-config rustc cargo openssl ];
        };
      });
}
