{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "rust-concurrency-patterns";
  version = "0.1.0";

  src = builtins.path {
    path = ./.;
    name = pname;
    filter = lib.cleanSourceFilter;
  };

  cargoSha256 = "sha256-4fNT4jvMf2rJiH+nFYOyWqgdueSjhxq6YHopjZG6taY=";
  verifyCargoDeps = false;  # FIXME: fails when it's set to true

  preferLocalBuild = true;
}
