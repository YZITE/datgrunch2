{
  description = "an encrypted messaging system";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-20.09";
    yz-flake-utils.url = "github:YZITE/flake-utils";
    # needed for default.nix, shell.nix
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { nixpkgs, yz-flake-utils, ... }:
    yz-flake-utils.lib.mkFlake {
      prevpkgs = nixpkgs;
      defaultProgName = "dg2";
      overlay = final: prev:
        let
          crates = (final.pkgs.callPackage ./Cargo.nix {
              defaultCrateOverrides = final.defaultCrateOverrides // {
                libsodium-sys = attrs: {
                  nativeBuildInputs = [ final.pkg-config ];
                  buildInputs = [ final.libsodium ];
                  SODIUM_USE_PKG_CONFIG = 1;
                };
              };
            }).workspaceMembers;
          crates2 = crates // {
              dg2core = crates.dg2core // { build = crates.dg2core.build.override {
                runTests = true;
              }; };
            };
          mkProg = name:
            {
              inherit name;
              value = crates2.${name}.build;
            };

        in builtins.listToAttrs (builtins.map mkProg ["dg2" "dg2core"]);
    };
}
