{
  lib,
  rustPlatform,
  fetchFromGitHub,
  installShellFiles,
}:

rustPlatform.buildRustPackage rec {
  pname = "ttrack";
  version = "0.3.1";
  useFetchCargoVendor = true;

  src = fetchFromGitHub {
    owner = "just1602";
    repo = pname;
    rev = "v${version}";
    hash = "sha256-gsJYWU2K57utAuqxbPRsdFxD4qpcl9vfB6I9D0EkgCU=";
  };

  cargoHash = "sha256-P6pcY/4Wv/RWYgOkJmMqpbELbY5+o+G9JS2DXeogB7A=";

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
