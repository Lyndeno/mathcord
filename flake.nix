{
  description = "A math bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];

    pkgs = import nixpkgs {
      inherit system overlays;
    };

    lib = nixpkgs.lib;
  in
  {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = [
        pkgs.rust-bin.stable.latest.default
      ];

    };
  };
}
