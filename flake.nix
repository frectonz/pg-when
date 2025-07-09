{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
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
            pkgs.rust-analyzer
            pkgs.rust-bin.stable.latest.default

            pkgs.nixfmt-rfc-style
          ];

          inputsFrom = with pkgs; [
            postgresql_13
            postgresql_14
            postgresql_15
            postgresql_16
            postgresql_17
          ];

          nativeBuildInputs = [
            pkgs.rustPlatform.bindgenHook
          ];
        };
      });

      packages = forAllSystems (
        pkgs:
        let
          pname = "pg-when";
          version = "0.1.0";

          buildPgliteFusionImage =
            {
              imageDigest,
              imageSha256,
              postgres,
            }:
            let
              postgresMajor = pkgs.lib.versions.major postgres.version;

              postgresImage = pkgs.dockerTools.pullImage {
                imageName = "postgres";
                imageDigest = imageDigest;
                sha256 = imageSha256;
              };

              extension = pkgs.stdenv.mkDerivation {
                pname = "${pname}-pg${postgresMajor}-extension";
                inherit version;

                src = import ./nix/build.nix {
                  inherit pkgs;
                  postgresql = postgres;
                  cargo-pgrx = pkgs.cargo-pgrx;
                };

                buildPhase = ''
                  install --directory $out/usr/share/postgresql/${postgresMajor}/extension
                  cp -r $src/nix/store/*/share/postgresql/extension/* $out/usr/share/postgresql/${postgresMajor}/extension
                  install --directory $out/usr/lib/postgresql/${postgresMajor}/lib
                  cp -r $src/nix/store/*/lib/* $out/usr/lib/postgresql/${postgresMajor}/lib
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

          pg13 = buildPgliteFusionImage {
            imageDigest = "sha256:80ff9e2086e68aef09839045df1f07016b869d94cbd12c6462a4b300878cfdac";
            imageSha256 = "sha256-iaUdJa/l0rgNkZR/FUoJ4bzmW/2CRWyk+eMHibBqIus=";
            postgres = pkgs.postgresql_13;
          };
          pg14 = buildPgliteFusionImage {
            imageDigest = "sha256:78b9deeca08fa9749a00e9d30bc879f8f8d021af854c73e2c339b752cb6d708a";
            imageSha256 = "sha256-LV2V6kuctIjN4gMxfopZSdivFtz7ks+AGmYQ4ets8b0=";
            postgres = pkgs.postgresql_14;
          };
          pg15 = buildPgliteFusionImage {
            imageDigest = "sha256:a35b3c0190dac5a82ec1778b34cb4963bdd9d161f80381a6297be6e2c3c13a7c";
            imageSha256 = "sha256-ZK6eBPA50mY99uSF3+UdT4eBm/3komc6sfWb1qw1N7k=";
            postgres = pkgs.postgresql_15;
          };
          pg16 = buildPgliteFusionImage {
            imageDigest = "sha256:5d65b8bdb20369ea902b987aa63cfe4983130bc8cd2c25830d126636b80b608d";
            imageSha256 = "sha256-5JhtZaCLj6SnJzjhC5A2yrP6fipuaQKHSxm3jhxSfNg=";
            postgres = pkgs.postgresql_16;
          };
          pg17 = buildPgliteFusionImage {
            imageDigest = "sha256:994cc3113ce004ae73df11f0dbc5088cbe6bb0da1691dd7e6f55474202a4f211";
            imageSha256 = "sha256-OzqtbX89/lBP2mzhSccuad5suUz/uw/gBgeIW3BTbdc=";
            postgres = pkgs.postgresql_17;
          };
        in
        {
          inherit
            pg13
            pg14
            pg15
            pg16
            pg17
            ;

          deploy = pkgs.writeShellScriptBin "deploy" ''
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg13} docker://ghcr.io/frectonz/${pname}:pg13-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg13-${version} docker://ghcr.io/frectonz/${pname}:pg13 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg14} docker://ghcr.io/frectonz/${pname}:pg14-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg14-${version} docker://ghcr.io/frectonz/${pname}:pg14 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg15} docker://ghcr.io/frectonz/${pname}:pg15-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg15-${version} docker://ghcr.io/frectonz/${pname}:pg15 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg16} docker://ghcr.io/frectonz/${pname}:pg16-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg16-${version} docker://ghcr.io/frectonz/${pname}:pg16 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker-archive:${pg17} docker://ghcr.io/frectonz/${pname}:pg17-${version} --dest-creds="frectonz:$ACCESS_TOKEN"
            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg17-${version} docker://ghcr.io/frectonz/${pname}:pg17 --dest-creds="frectonz:$ACCESS_TOKEN"

            ${pkgs.skopeo}/bin/skopeo --insecure-policy copy docker://ghcr.io/frectonz/${pname}:pg17 docker://ghcr.io/frectonz/${pname}:latest --dest-creds="frectonz:$ACCESS_TOKEN"
          '';
        }
      );

      formatter = forAllSystems (pkgs: pkgs.treefmt);
    };
}
