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
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      cross-toolchain = with fenix.packages.${system}; combine [
        minimal.cargo
        minimal.rustc
        targets.wasm32-unknown-unknown.latest.rust-std
      ];
      cross-compile = naersk.lib.${system}.override {
        cargo = cross-toolchain;
        rustc = cross-toolchain;
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
    {
      defaultPackage = cross-compile.buildPackage {
        src = ./.;
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
      };
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
    });
}
