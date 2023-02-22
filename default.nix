{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "rust shell";
  buildInputs = with pkgs; [ 
    rustc cargo rustfmt clippy
    (callPackage /etc/nixos/vim.nix {
      extraPlugins = with vimPlugins; [
        rust-tools-nvim
      ];
    })
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
