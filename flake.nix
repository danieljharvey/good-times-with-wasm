{
  description = "Rusty";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        packageName = "rusty";
      in {
        # we're not interested in building with Nix, just using it for deps
        packages.${system}.${packageName} = {};

        defaultPackage = self.packages.${system}.${packageName};

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            cargo-watch
            rustfmt
            libiconv
          ];

          # put clang_14 on the path
          shellHook = with pkgs; ''
            export PATH="${clang_14}/bin:$PATH"
          '';

          inputsFrom = builtins.attrValues self.packages.${system};
        };
      });
}


