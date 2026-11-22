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
            postgresql_15
            postgresql_16
            postgresql_17
            postgresql_18
            postgresql_19
          ];

          nativeBuildInputs = [
            pkgs.rustPlatform.bindgenHook
          ];

          # clang needs an explicit SDK sysroot for pgrx-bindgen and the pgrx cshim
          shellHook = pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isDarwin ''
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

          pg15 = buildPgWhenImage {
            imageDigest = "sha256:8cf673eb99f8b9042c9078ee4ca0e775428c77097e9d999bfc9ba4b4347f5733";
            imageSha256 = "sha256-Or21OsMu8GE4HbDYkRVmIxSirEQ3+8qh5kquBJtWMmE=";
            postgresDev = pkgs.postgresql_15;
          };
          pg16 = buildPgWhenImage {
            imageDigest = "sha256:3950ad7c3148ca2175ef1b6c36c9efbf78957ae8fbe8b3a5d7c35af50500382f";
            imageSha256 = "sha256-t8OUAl53wqyT7diqlTCuDEg5MOytSozsWU4MUZ3oCG4=";
            postgresDev = pkgs.postgresql_16;
          };
          pg17 = buildPgWhenImage {
            imageDigest = "sha256:223efeb196329b55b3b9a13db144956411dbc18e727e59f681c3e0aede6475cf";
            imageSha256 = "sha256-P9yM6f8N8x0ZZkP0b+384S6v+643qO8rlnA2z0BHBg0=";
            postgresDev = pkgs.postgresql_17;
          };
          pg18 = buildPgWhenImage {
            imageDigest = "sha256:eb63372c1e05c755f55fa30cbd07aecc443676fc6c0fa90b9002d01537841e1a";
            imageSha256 = "sha256-7XOw/2ppX8H790nDYtgQ6fmv8cidZ99w4S1Fj9milV8=";
            postgresDev = pkgs.postgresql_18;
          };
          pg19 = buildPgWhenImage {
            imageDigest = "sha256:bb6e70add742cd868fb62cb3b2f67d1dd2bdd2b65a521e778420748753687c60";
            imageSha256 = "sha256-ssZSnSbzpqpRVHLlXtrefz4L3BGyBnRJwBcKTEtx4Q0=";
            postgresDev = pkgs.postgresql_19;
          };
        in
        {
          inherit
            pg15
            pg16
            pg17
            pg18
            pg19
            ;

          deploy = pkgs.writeShellScriptBin "deploy" ''
            # CI runners ship a v1 /etc/containers/registries.conf, which skopeo no longer loads
            export CONTAINERS_REGISTRIES_CONF="${pkgs.writeText "registries.conf" ''
              unqualified-search-registries = [ "docker.io" ]
            ''}"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg15} docker://docker.io/frectonz/${pname}:pg15-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg15-${version} docker://docker.io/frectonz/${pname}:pg15 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg16} docker://docker.io/frectonz/${pname}:pg16-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg16-${version} docker://docker.io/frectonz/${pname}:pg16 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg17} docker://docker.io/frectonz/${pname}:pg17-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg17-${version} docker://docker.io/frectonz/${pname}:pg17 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg18} docker://docker.io/frectonz/${pname}:pg18-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg18-${version} docker://docker.io/frectonz/${pname}:pg18 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg19} docker://docker.io/frectonz/${pname}:pg19-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://docker.io/frectonz/${pname}:pg19-${version} docker://docker.io/frectonz/${pname}:pg19 --dest-creds="frectonz:$ACCESS_TOKEN"

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
