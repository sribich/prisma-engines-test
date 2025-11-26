{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      mkPkgs =
        repository: system: overlays:
        import repository {
          inherit system;
          overlays = overlays;

          allowBroken = true;
          config = {
            allowBroken = true;
          };
        };

      mkShell =
        system:
        let
          pkgs = mkPkgs nixpkgs system ([
            (import rust-overlay)
            (final: prev: {
              unstable = mkPkgs nixpkgs system [ ];
            })
          ]);

        in
        import ./shell.nix {
          inherit
            self
            system
            pkgs
            ;
        };
    in
    {
      devShells."x86_64-linux" = mkShell "x86_64-linux";
    };
}
