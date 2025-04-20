{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
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
    rust-overlay,
    treefmt-nix,
    ...
  }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
    };
    toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
      toolchain.default.override {
        extensions = ["rust-src"];
        targets = ["riscv64gc-unknown-none-elf"];
      });
    treefmt = (treefmt-nix.lib.evalModule pkgs (import ./treefmt.nix {inherit pkgs;})).config.build;
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = [toolchain pkgs.qemu];
    };
    formatter.x86_64-linux = treefmt.wrapper;
  };
}
