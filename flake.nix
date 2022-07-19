{
  inputs = {
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = { self, nixpkgs, flake-utils, nix-filter, naersk, fenix }: flake-utils.lib.eachDefaultSystem
    (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        wasm-toolchain = with fenix.packages.${system}; combine [
          minimal.cargo
          minimal.rustc
          targets.wasm32-unknown-unknown.latest.rust-std
        ];
        rust-wasm = naersk.lib.${system}.override {
          cargo = wasm-toolchain;
          rustc = wasm-toolchain;
        };
        dev-toolchain = fenix.packages.${system}.complete.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-src"
        ];
        darwin-support =
          if pkgs.stdenv.isDarwin then with pkgs.darwin.apple_sdk.frameworks; [ Security AppKit ] else [ ];
      in
      rec {
        packages = {
          wasm = rust-wasm.buildPackage {
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            src = nix-filter.lib {
              root = ./.;
              include = [
                "Cargo.lock"
                "Cargo.toml"
                (nix-filter.lib.inDirectory "src")
              ];
            };
          };
          web-app = pkgs.runCommand "roguelike-web-app" { } ''
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen ${packages.wasm}/bin/roguelike.wasm --out-dir $out/web-app/wasm --no-modules --no-typescript
            cp ${./public/index.html} $out/web-app/index.html
          '';
        };
        defaultPackage = packages.web-app;
        devShell = pkgs.mkShell {
          buildInputs =
            [
              dev-toolchain
              pkgs.libiconv
              pkgs.sqlite
              pkgs.lldb
              pkgs.wasm-bindgen-cli
              pkgs.wasm-pack
              pkgs.simple-http-server
            ] ++ darwin-support;

          RUST_SRC_PATH = "${dev-toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
