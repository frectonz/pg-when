self: super:

{
  cargo-pgrx' = super.rustPlatform.buildRustPackage rec {
    pname = "cargo-pgrx";
    version = "0.15.0";

    src = super.fetchCrate {
      inherit version;
      pname = "cargo-pgrx";
      hash = "sha256-sksRfNV6l8YbdI6fzrEtanpDVV4sh14JXLqYBydHwy0=";
    };

    cargoHash = "sha256-c+n1bJMO9254kT4e6exVNhlIouzkkzrRIOVzR9lZeg4=";

    nativeBuildInputs = [
      super.pkg-config
    ];

    buildInputs = [
      super.openssl
    ];

    preCheck = ''
      export PGRX_HOME=$(mktemp -d)
    '';

    checkFlags = [
      # requires pgrx to be properly initialized with cargo pgrx init
      "--skip=command::schema::tests::test_parse_managed_postmasters"
    ];
  };
}
