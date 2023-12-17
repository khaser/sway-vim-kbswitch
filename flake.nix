{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    khaser.url = "git+ssh://git@109.124.253.149/~git/nixos-config?ref=master";
  };

  outputs = { self, nixpkgs, flake-utils, khaser }:
  flake-utils.lib.eachDefaultSystem ( system:
  let
    pkgs = import nixpkgs { inherit system; };
  in {

    devShell = pkgs.mkShell {
      name = "rust-vim";

      inputsFrom = [ self.outputs.defaultPackage ];

      buildInputs = with pkgs; [
        rustc
        cargo
        rustfmt
        clippy
        rust-analyzer
        (callPackage khaser.lib.vim {
          extraPlugins = with vimPlugins; [
            rust-vim
            tagbar
            syntastic
            LanguageClient-neovim
          ];
          extraRC = ''
            let g:rustfmt_autosave = 1

            let g:LanguageClient_serverCommands = {
             \ 'rust': ['rust-analyzer']
             \ }
            nnoremap <F5> :call LanguageClient_contextMenu()<CR>
            nnoremap <silent> gh :call LanguageClient_textDocument_hover()<CR>
            nnoremap <silent> gd :call LanguageClient_textDocument_definition()<CR>
            nnoremap <silent> gr :call LanguageClient_textDocument_references()<CR>
            nnoremap <silent> gs :call LanguageClient_textDocument_documentSymbol()<CR>
            nnoremap <silent> <F2> :call LanguageClient_textDocument_rename()<CR>
            nnoremap <silent> gf :call LanguageClient_textDocument_formatting()<CR>
          '';
        })
      ];

      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    };

    defaultPackage = let
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in
      pkgs.rustPlatform.buildRustPackage (with manifest; {
        inherit name version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta = {
          inherit description license;
          homepage = repository;
        };
      });
  });

}
