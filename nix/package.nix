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
    mkdir -p completions
    $out/bin/ttrack completion bash > completions/ttrack.bash
    $out/bin/ttrack completion zsh > completions/_ttrack
    $out/bin/ttrack completion fish > completions/ttrack.fish
    installShellCompletion --bash completions/ttrack.bash --zsh completions/_ttrack --fish completions/ttrack.fish
  '';

  meta = {
    description = "A small CLI time tracker";
    homepage = "https://github.com/just1602/ttrack";
    license = lib.licenses.gpl3Only;
    mainProgram = "ttrack";
  };
}
