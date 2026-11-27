{
  lib,
  rustPlatform,
  installShellFiles,
  ...
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
in
rustPlatform.buildRustPackage {
  inherit pname version;

  src = ./..;
  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installShellCompletion --cmd ttrack\
      --bash <($out/bin/ttrack completion bash) \
      --zsh <($out/bin/ttrack completion zsh) \
      --fish <($out/bin/ttrack completion fish)
  '';

  meta = {
    description = "A small CLI time tracker";
    homepage = "https://github.com/just1602/ttrack";
    license = lib.licenses.gpl3Only;
    mainProgram = "ttrack";
  };
}
