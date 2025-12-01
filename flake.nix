{
  description = "Basic Rust Development Flake For Gamers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {

   devShells."x86_64-linux".default = pkgs.mkShell {
     buildInputs = with pkgs; [
       fish neovim cargo rustc rustfmt clippy rust-analyzer glib
     ];

     nativeBuildInputs = [pkgs.pkg-config ];

     shellHook = ''
     exec fish
     '';

     env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
   };

  };
}
