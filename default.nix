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

  cargoSha256 = "0qqap1f8wmbiz6dn8q6q4zd8skxqnyszqjyg4bw4shvv422933s2";
  verifyCargoDeps = false;  # FIXME: fails when it's set to true

  preferLocalBuild = true;
}
