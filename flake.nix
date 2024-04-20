{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
	  nativeBuildInputs = [ pkg-config ];
          buildInputs = [ cargo rustc rustfmt rust-analyzer rustup pre-commit rustPackages.clippy # Rust
			gobject-introspection-unwrapped glib gdk-pixbuf gtk4 pango libxml2 librsvg libadwaita # GTK
			bacon neovim neovide ]; # Me
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
