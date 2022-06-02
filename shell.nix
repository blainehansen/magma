{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/52dc75a4fee3fdbcb792cb6fba009876b912bfe0.tar.gz") {} }:

let
  coq = pkgs.coq.override { version = "cbe681ab1a9db43e28327716a76db4dee5adc2e2"; customOCamlPackages = pkgs.ocamlPackages; };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    coq
    dune_3
    inotify-tools
    just
    libffi
    libiconv
    llvmPackages_13.llvm
    (runCommand "lli-13" {} "mkdir -p $out/bin && ln -s ${llvmPackages_13.llvm}/bin/lli $out/bin/lli-13")
    ocaml
    ocamlformat
    opam
    rustc
  ] ++ (with ocamlPackages; [
    findlib
    alcotest
    merlin
    ocaml-lsp
    ppx_assert
    ppx_inline_test
    sexplib
    zarith
  ]);

  RUSTFLAGS = "-l LLVM-13";
}
