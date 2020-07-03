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

  cargoSha256 = "05lf2bjnqd3jggpfly3skh686is57gk81x8hcdwsyyjsvcxs0ixp";
  verifyCargoDeps = false;

  preferLocalBuild = true;
}
