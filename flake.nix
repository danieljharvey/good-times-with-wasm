{
  description = "Rusty";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        pkgs = import nixpkgs {
          system = localSystem;
          overlays = [ rust-overlay.overlays.default ];
        };
        packageName = "rusty";
      in
      {
        # we're not interested in building with Nix, just using it for deps
        packages.${localSystem}.${packageName} = { };

        defaultPackage = self.packages.${localSystem}.${packageName};

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            just
            cargo-watch
            rustfmt
            libiconv
          ];

          # put clang_14 on the path
          shellHook = with pkgs; ''
            export PATH="${clang_14}/bin:$PATH"
          '';

          inputsFrom = builtins.attrValues self.packages.${localSystem};
        };
      });
}
