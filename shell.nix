{ rust-bin
, mkShell
, pkg-config
, openssl
}: mkShell {
  packages = [
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
      openssl
      pkg-config
  ];
}
