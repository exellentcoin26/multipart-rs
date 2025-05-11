{
  inputs = {
    nixpkgs.url = "nixpkgs/release-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # (rust-bin.stable."1.75.0".default.override {
            #   extensions = ["rust-src" "rust-analyzer"];
            # })

            (rust-bin.nightly.latest.default.override {
              extensions = ["rust-src" "rust-analyzer"];
            })

            cargo-machete
            cargo-fuzz
          ];
        };
      }
    );
}
