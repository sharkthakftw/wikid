{
  description = "feature-rich terminal wikipedia client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
          }
        );
    in
    {
      packages = forAllSystems (
        { pkgs, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          wikid = pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            src = pkgs.lib.cleanSource ./.;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = [ pkgs.installShellFiles ];

            postInstall = ''
              installShellCompletion --fish completions/wikid.fish
            '';

            meta = {
              description = cargoToml.package.description;
              homepage = cargoToml.package.repository;
              license = pkgs.lib.licenses.mit;
              mainProgram = "wikid";
              platforms = pkgs.lib.platforms.unix;
            };
          };
        in
        {
          inherit wikid;
          default = wikid;
        }
      );

      apps = forAllSystems (
        { pkgs, system }:
        {
          default = {
            type = "app";
            program = pkgs.lib.getExe self.packages.${system}.wikid;
            meta = {
              inherit (self.packages.${system}.wikid.meta) description;
            };
          };
        }
      );

      devShells = forAllSystems (
        { pkgs, ... }:
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.rustc
              pkgs.clippy
              pkgs.rustfmt
              pkgs.rust-analyzer
            ];

            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      overlays.default = final: _prev: {
        wikid = self.packages.${final.stdenv.hostPlatform.system}.wikid;
      };
    };
}
