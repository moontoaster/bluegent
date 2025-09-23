{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
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
          bluegent =
            let
              c2n = import ./nix/Crate.nix {
                pkgs = final;
              };
            in
            c2n.rootCrate.build;
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
            crate2nix
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });
    };
}
