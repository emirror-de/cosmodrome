{
  description = "rocket-airport development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils = { url = "github:numtide/flake-utils"; };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        devShell = pkgs.mkShell {
          name = "rocket-airport development";
          buildInputs = [
            pkgs.nushell
            pkgs.pkg-config
            pkgs.openssl
          ];
          shellHook = ''
            export RUSTUP_TOOLCHAIN=nightly
            zellij --layout layout.kdl
            exit
          '';
        };
      });
}
