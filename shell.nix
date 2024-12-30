{ pkgs ? import <nixpkgs> {}
}:


pkgs.mkShell {
  buildInputs = [
    pkgs.eza
    pkgs.git
    pkgs.neovim
    pkgs.rustc
    pkgs.rustup
    pkgs.rust-analyzer
    pkgs.rustfmt
    pkgs.ruby_3_4
  ];
}
