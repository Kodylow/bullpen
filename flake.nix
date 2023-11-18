{
  description =
    "Pplx Client: an unofficial async Rust library for interacting with the Perplexity API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        flakeboxLib = flakebox.lib.${system} { };
        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "pplx-api";
            path = ./.;
          };
          paths = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };

        outputs = (flakeboxLib.craneMultiBuild { }) (craneLib':
          let
            craneLib = (craneLib'.overrideArgs {
              pname = "flexbox-multibuild";
              src = rustSrc;
            });
          in rec {
            workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
            workspaceBuild =
              craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
            pplx-api = craneLib.buildPackage { };
          });
      in {
        devShells = flakeboxLib.mkShells {
          packages = [ ];
          buildInputs = [
            nixpkgs.starship
            nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            nixpkgs.pkg-config
          ];
          shellHook = ''
            eval "$(starship init bash)"
          '';
        };
      });
}
