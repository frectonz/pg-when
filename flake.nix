{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/*";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      version = (nixpkgs.lib.importTOML ./Cargo.toml).package.version;

      forAllSystems =
        fn:
        let
          systems = [
            "x86_64-linux"
            "aarch64-darwin"
          ];
          overlays = [ (import rust-overlay) ];
        in
        nixpkgs.lib.genAttrs systems (
          system:
          fn (
            import nixpkgs {
              inherit system overlays;
            }
          )
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = [
            pkgs.bacon
            pkgs.cargo-pgrx
            pkgs.cargo-insta
            pkgs.cargo-outdated
            pkgs.rust-analyzer
            pkgs.rust-bin.stable.latest.default
          ];

          inputsFrom = with pkgs; [
            postgresql_14
            postgresql_15
            postgresql_16
            postgresql_17
            postgresql_18
          ];

          nativeBuildInputs = [
            pkgs.rustPlatform.bindgenHook
          ];

          # clang needs an explicit SDK sysroot for pgrx-bindgen and the pgrx cshim
          shellHook = pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
            export BINDGEN_EXTRA_CLANG_ARGS="-isysroot $SDKROOT"
            export CFLAGS="-isysroot $SDKROOT"
          '';
        };
      });

      packages = forAllSystems (
        pkgs:
        let
          pname = "pg-when";

          buildPgWhenImage =
            {
              imageDigest,
              imageSha256,
              postgresDev,
            }:
            let
              postgresMajor = pkgs.lib.versions.major postgresDev.version;

              postgresImage = pkgs.dockerTools.pullImage {
                imageName = "postgres";
                imageDigest = imageDigest;
                sha256 = imageSha256;
                os = "linux";
                arch = "amd64";
              };

              extension = pkgs.stdenv.mkDerivation {
                pname = "${pname}-pg${postgresMajor}-extension";
                inherit version;

                src = import ./nix/build.nix {
                  inherit pkgs version;
                  postgresql = postgresDev;
                };

                buildPhase = ''
                  install --directory $out/usr/share/postgresql/${postgresMajor}/extension
                  cp -r $src/share/postgresql/extension/* $out/usr/share/postgresql/${postgresMajor}/extension
                  install --directory $out/usr/lib/postgresql/${postgresMajor}/lib
                  cp -r $src/lib/* $out/usr/lib/postgresql/${postgresMajor}/lib
                '';
              };
            in
            pkgs.dockerTools.buildLayeredImage {
              name = "${pname}-pg${postgresMajor}";
              fromImage = postgresImage;

              contents = [ extension ];
              config = {
                Env = [ "POSTGRES_HOST_AUTH_METHOD=trust" ];

                Expose = 5432;
                Cmd = [ "postgres" ];
                Entrypoint = [ "docker-entrypoint.sh" ];
              };
            };

          pg14 = buildPgWhenImage {
            imageDigest = "sha256:2f439458ab6a57a925825ae14f9d06910e4fe4a41c8d4a0ae06397e65b707e1b";
            imageSha256 = "sha256-nUH9eeEEOzM6TQ0lR2TV4vSeHOP8EBnydPw1pmYkBlg=";
            postgresDev = pkgs.postgresql_14;
          };
          pg15 = buildPgWhenImage {
            imageDigest = "sha256:6eb0add3b77c081df18aa518ce43df58fdcc40f2e6d868a6fd08038dc7acd425";
            imageSha256 = "sha256-1wRjSCsyfhuCAsB4H8vmoVdbRGwb26uNEBTkb1en7zs=";
            postgresDev = pkgs.postgresql_15;
          };
          pg16 = buildPgWhenImage {
            imageDigest = "sha256:95206741a5b214807675e14165369d05b93a9cf692223b616d07cca227e74b0b";
            imageSha256 = "sha256-fOiVQPp4cWTNqOageKAmtrHYEve+cPvbvsZ30LUsXwY=";
            postgresDev = pkgs.postgresql_16;
          };
          pg17 = buildPgWhenImage {
            imageDigest = "sha256:7958605b474b3d264a969cb3a123d6aa00ad1e1fe9da8a69984dabb704d93317";
            imageSha256 = "sha256-jdFum5FV0x6HS4/NHiK1UwRKvUu+POIWxwSlBlK99AE=";
            postgresDev = pkgs.postgresql_17;
          };
          pg18 = buildPgWhenImage {
            imageDigest = "sha256:a02db8cac496f15b094798a38254f14d6e00741f709360e5e00bb6668ea31636";
            imageSha256 = "sha256-IPMO8ywiLVd9xP5QGmUWvno62YAmPuZpWzN2a8e14Gs=";
            postgresDev = pkgs.postgresql_18;
          };
        in
        {
          inherit
            pg14
            pg15
            pg16
            pg17
            pg18
            ;

          deploy = pkgs.writeShellScriptBin "deploy" ''
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg14} docker://docker.io/frectonz/${pname}:pg14-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg14-${version} docker://docker.io/frectonz/${pname}:pg14 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg15} docker://docker.io/frectonz/${pname}:pg15-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg15-${version} docker://docker.io/frectonz/${pname}:pg15 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg16} docker://docker.io/frectonz/${pname}:pg16-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg16-${version} docker://docker.io/frectonz/${pname}:pg16 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg17} docker://docker.io/frectonz/${pname}:pg17-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg17-${version} docker://docker.io/frectonz/${pname}:pg17 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg18} docker://docker.io/frectonz/${pname}:pg18-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg18-${version} docker://docker.io/frectonz/${pname}:pg18 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg18 docker://docker.io/frectonz/${pname}:latest --dest-creds="frectonz:$ACCESS_TOKEN"
          '';
        }
      );

      formatter = forAllSystems (
        pkgs:
        pkgs.treefmt.withConfig {
          runtimeInputs = [ pkgs.nixfmt ];

          settings = {
            on-unmatched = "info";
            formatter.nixfmt = {
              command = "nixfmt";
              includes = [ "*.nix" ];
            };
          };
        }
      );
    };
}
