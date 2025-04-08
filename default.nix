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

  cargoHash = "sha256-Ecs9W3tSbjuL6f9jZRfaGHzRsbK1LN/2WkOK0dHN1ww=";
  verifyCargoDeps = false;  # FIXME: fails when it's set to true

  preferLocalBuild = true;
}
