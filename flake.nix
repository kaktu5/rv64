{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    treefmt-nix,
    ...
  }:
    flake-utils.lib.eachSystem ["aarch64-linux" "x86_64-linux"] (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit overlays system;};
      toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          extensions = ["rust-src"];
          targets = ["riscv64gc-unknown-none-elf"];
        });
      treefmt =
        (treefmt-nix.lib.evalModule pkgs (
          import ./treefmt.nix {inherit pkgs;}
        )).config.build;
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [toolchain] ++ (with pkgs; [gdb qemu]);
      };
      formatter = treefmt.wrapper;
    });
}
