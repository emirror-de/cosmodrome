{
  description = "rocket-airport development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils = { url = "github:numtide/flake-utils"; };
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem ["aarch64-darwin" "x86_64-linux"] (system: 
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        # system specifics
        system_pkgs = {
          aarch64-darwin = with pkgs; [
            libiconv
          ];
          x86_64-linux = [
          ];
        };
      in
      {
        devShell = pkgs.mkShell {
          name = "cosmodrome development";
          buildInputs = [
            pkgs.nushell
            pkgs.pkg-config
            pkgs.openssl
          ] ++ system_pkgs.${system};
          shellHook = ''
            export RUSTUP_TOOLCHAIN=nightly
            zellij --layout layout.kdl
            exit
          '';
        };
      });
}
