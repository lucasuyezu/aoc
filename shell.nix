{ pkgs ? import <nixpkgs> {}
}:


pkgs.mkShell {
  buildInputs = [
    pkgs.vim
    pkgs.rustc
    pkgs.rustup
    pkgs.rust-analyzer
    pkgs.rustfmt
    pkgs.git
  ];
}
