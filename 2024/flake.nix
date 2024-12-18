{
  description = "A Devshell for rust development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixNvim = { url = "github:NeilDarach/nixNvim"; };

  };
  outputs = { self, nixpkgs, flake-utils, nixNvim, rust-overlay, ... }@inputs:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = let rust = pkgs.rust-bin;
        in if builtins.pathExists ./rust-toolchain.toml then
          rust.fromRustupToolchainFile ./rust-toolchain.toml
        else if builtins.pathExists ./rust-toolchain then
          rust.fromRustToolchainFile ./rust-toolchain
        else
          rust.nightly.latest.default.override {
            extensions = [ "rust-src" "rustfmt" "rust-analyzer" ];
          };

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustToolchain
              just
              bacon
              tracy
              openssl
              pkg-config
              clippy
              cargo-generate
              cargo-watch
              cargo-nextest
              cargo-flamegraph
              nixNvim.packages.${pkgs.system}.nvim
            ] ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];
          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
      }));
}
