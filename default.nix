{ rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "bluegent";
  version = "0.0.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
