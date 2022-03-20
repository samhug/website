{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs {
    overlays = [
      (_: _: { inherit sources; })
    ];
  };
in
pkgs
