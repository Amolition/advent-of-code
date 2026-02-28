{
  description = "Rust Dev Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    {
      devShells."${system}".default =
        with pkgs;
        mkShell {
          packages = [
            # core
            openssl
            pkg-config
            rust-bin.stable.latest.default
            rust-analyzer
            rustfmt
            lldb_21
            # rustlings
            rustlings
          ];

          shellHook = ''
            rustc --version
            cargo --version
          '';
        };
    };
}
