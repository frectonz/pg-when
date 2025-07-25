{
  pkgs,
  cargo-pgrx,
  postgresql,
}:

let
  postgresMajor = pkgs.lib.versions.major postgresql.version;

  preBuildAndTest = ''
    export PGRX_HOME=$(mktemp -d)
    mkdir -p $PGRX_HOME/${postgresMajor}

    cp -r -L ${postgresql.dev}/. $PGRX_HOME/${postgresMajor}/
    chmod -R ugo+w $PGRX_HOME/${postgresMajor}
    cp -r -L ${postgresql.dev.lib}/lib/. $PGRX_HOME/${postgresMajor}/lib/
    cp -r -L ${postgresql.pg_config}/bin/. $PGRX_HOME/${postgresMajor}/bin/

    ${cargo-pgrx}/bin/cargo-pgrx pgrx init \
      --pg${postgresMajor} $PGRX_HOME/${postgresMajor}/bin/pg_config
  '';

  filterCargoSources =
    orig_path: type:
    let
      path = (toString orig_path);
      base = baseNameOf path;
      parentDir = baseNameOf (dirOf path);

      matchesSuffix = pkgs.lib.any (suffix: pkgs.lib.hasSuffix suffix base) [
        ".rs"
        ".toml"
        ".control"
      ];

      # Cargo.toml already captured above
      isCargoFile = base == "Cargo.lock";

      # .cargo/config.toml already captured above
      isCargoConfig = parentDir == ".cargo" && base == "config";
    in
    (type == "directory") || matchesSuffix || isCargoFile || isCargoConfig;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "pg-when";
  version = "0.1.9";

  src = pkgs.lib.cleanSourceWith {
    src = pkgs.lib.cleanSource ../.;
    filter = filterCargoSources;
  };
  cargoLock.lockFile = ../Cargo.lock;

  doCheck = false;
  buildNoDefaultFeatures = true;
  buildFeatures = [ "pg${postgresMajor}" ];

  preBuild = preBuildAndTest;
  preCheck = preBuildAndTest;
  postPatch = "patchShebangs .";

  postBuild = ''
    if [ -f "pg_when.control" ]; then
      export NIX_PGLIBDIR=${postgresql.dev.out}/share/postgresql/extension/
      ${cargo-pgrx}/bin/cargo-pgrx pgrx package --pg-config ${postgresql.pg_config}/bin/pg_config --out-dir the-thing
      export NIX_PGLIBDIR=$PGRX_HOME/${postgresMajor}/lib
    fi
  '';

  preFixup = ''
    if [ -f "pg_when.control" ]; then
      ${cargo-pgrx}/bin/cargo-pgrx pgrx stop all
      rm -rfv $out/target*
    fi
  '';

  installPhase = ''
    cp -rp the-thing $out
  '';

  nativeBuildInputs = [
    postgresql.dev
    pkgs.rustfmt
    postgresql.lib
    pkgs.pkg-config
    pkgs.rustPlatform.bindgenHook
  ];

  PGRX_PG_SYS_SKIP_BINDING_REWRITE = "1";
}
