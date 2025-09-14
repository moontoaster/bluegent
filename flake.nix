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
          pkgs = import nixpkgs { inherit system; };
        in
        f pkgs;

      forEachSystem = f: genAttrs supportedSystems (system: instantiateSystem f system);
    in
    {
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
