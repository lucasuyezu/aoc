{ pkgs ? import <nixpkgs> {}
}:


pkgs.mkShell {
  buildInputs = [
    pkgs.exa
    pkgs.git
    pkgs.neovim
    pkgs.rustc
    pkgs.rustup
    pkgs.rust-analyzer
    pkgs.rustfmt
  ];
}
