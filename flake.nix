# Created with reference to https://github.com/modrinth/sisyphus/blob/master/flake.nix
{
  description = "Greenback Workers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };

      toolchain = with fenix.packages.${system}; combine [
        minimal.cargo
        minimal.rustc
        default.clippy
        targets.wasm32-unknown-unknown.latest.rust-std
      ];
    in {
      devShells.default = pkgs.mkShell {
        shellHook = ''
          export PATH+=:~/.cargo/bin
          export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib64:${pkgs.zlib}/lib:$LD_LIBRARY_PATH"
          alias nix-miniflare="npx miniflare"
        '';
        buildInputs = with pkgs; [
          wasm-pack wrangler toolchain wasm-bindgen-cli
          git
					# needed for miniflare
          nodejs-18_x
        ];
      };
    });
}
