{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem ( system:
  let
    pkgs = import nixpkgs { inherit system; };
  in {
    defaultPackage = let
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in
      pkgs.rustPlatform.buildRustPackage (with manifest; {
        inherit name version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta = {
          inherit description license;
          homepage = repository;
        };
      });
  });

}
