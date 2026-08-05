{
  description = "App-agnostic utilities for Dioxus web apps";

  nixConfig = {
    extra-substituters = ["https://clemenscodes.cachix.org"];
    extra-trusted-public-keys = [
      "clemenscodes.cachix.org-1:yEwW1YgttL2xdsyfFDz/vv8zZRhRGMeDQsKKmtV1N18="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        # Rust toolchain — version, targets, and components declared in
        # rust-toolchain.toml; fenix reads from there.
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-h+t2xTBz5yt2YIO+1VMIIGlCU7gyp2LYOFvaV1nwOXU=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # cleanCargoSource drops every non-Rust file, which would take
        # rustfmt.toml, clippy.toml and deny.toml with it. The first two would
        # leave the fmt and clippy gates judging this tree by defaults instead
        # of by its own settings. The third is worse, because cargo deny falls
        # back to its own policy without saying so and the license gate then
        # passes for a reason that has nothing to do with the list it was meant
        # to enforce.
        src = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = pkgs.lib.fileset.unions [
            (craneLib.fileset.commonCargoSources ./.)
            ./rustfmt.toml
            ./clippy.toml
            ./deny.toml
          ];
        };

        commonArgs = {
          inherit src;
          pname = "dioxus-kit";
          version = "0.1.0";
          strictDeps = true;
          cargoExtraArgs = "--workspace";
        };

        # Cache cargo dependencies separately so a code-only change doesn't
        # rebuild the world.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        dioxusKit = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});

        cargoFmt = craneLib.cargoFmt {inherit src;};

        cargoClippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });

        cargoTest = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});

        # This crate ends up in the dependency tree of a product that is sold,
        # so the license of everything it drags along is a build failure rather
        # than something to notice later. deny.toml holds the list.
        cargoDeny = craneLib.cargoDeny {
          inherit src;
          cargoDenyChecks = "licenses";
        };
      in {
        formatter = pkgs.alejandra;

        packages = {
          default = dioxusKit;
          inherit dioxusKit cargoArtifacts;
        };

        checks = {
          inherit dioxusKit cargoFmt cargoClippy cargoTest cargoDeny;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [dioxusKit];
          packages = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-nextest
            taplo
            alejandra
            nil
          ];

          shellHook = ''
            echo ""
            echo "  dioxus-kit — app-agnostic Dioxus utilities dev shell"
            echo ""
          '';
        };
      }
    );
}
