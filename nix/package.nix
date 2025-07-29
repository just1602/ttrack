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
    installShellCompletion --bash "dist/ttrack.bash" --zsh "dist/_ttrack" --fish "dist/ttrack.fish"
  '';

  meta = {
    description = "A small CLI time tracker";
    homepage = "https://github.com/just1602/ttrack";
    license = lib.licenses.gpl3Only;
    mainProgram = "ttrack";
  };
}
