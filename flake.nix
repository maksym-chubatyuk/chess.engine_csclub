{
  description = "Devenv for the CDL CS club chess engine";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };
      f = fenix.packages.${system};
      rustToolchain = f.combine [
        f.stable.toolchain
        f.targets.aarch64-apple-darwin.stable.rust-std
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          python314
          python314Packages.python-lsp-server
          python314Packages.numpy
          python314Packages.pytest
          pyright

          rustToolchain

          maturin
          auditwheel

          # For cross compiling
          zig
        ];

        shellHook = ''
          [ -d .venv ] || python -m venv .venv --copies
          source .venv/bin/activate
          # This causes bad wheel naming when cross compiling from linux to mac for some reason
          unset _PYTHON_HOST_PLATFORM
        '';
      };
    };
}
