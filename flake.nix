{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
  };

  nixConfig = {
    extra-substituters = [ "https://bluegent.cachix.org" ];
    extra-trusted-public-keys = [
      "bluegent.cachix.org-1:bXF6hBSwit6YXZ/SdlAU/pgDaYx3uVCI8BkxBJSL/bY="
    ];
  };

  outputs =
    { self, nixpkgs }:
    let
      inherit (nixpkgs.lib) genAttrs;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      instantiateSystem =
        f: system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlays.default ];
          };
        in
        f pkgs;

      forEachSystem = f: genAttrs supportedSystems (system: instantiateSystem f system);
    in
    {
      overlays = rec {
        bluegent = final: prev: {
          bluegent = final.callPackage ./nix/pkg.nix { };
        };

        default = bluegent;
      };

      packages = forEachSystem (pkgs: rec {
        inherit (pkgs) bluegent;
        default = bluegent;
      });

      nixosModules = rec {
        bluegent = import ./nix/nixos-module.nix;
        default = bluegent;
      };

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });
    };
}
