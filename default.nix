{ rustPlatform
, pkg-config
, openssl
}: rustPlatform.buildRustPackage {
  pname = "neige";
  version = "0.1.0";
  src = ./.;
  cargoLock = { lockFile = ./Cargo.lock; };
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}
