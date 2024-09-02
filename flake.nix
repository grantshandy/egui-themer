{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in {
        devShells.default = pkgs.mkShell rec {
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [ "rust-src" "rust-analyzer" ];
          };

          buildInputs = with pkgs; [ trunk rustToolchain cargo-watch ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ libGL wayland libxkbcommon ];
        };
      });
}
