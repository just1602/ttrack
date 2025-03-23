{
  lib,
  rustPlatform,
  fetchFromGitHub,
  installShellFiles,
}:

rustPlatform.buildRustPackage rec {
  pname = "ttrack";
  version = "0.3.2";
  useFetchCargoVendor = true;

  src = fetchFromGitHub {
    owner = "just1602";
    repo = pname;
    rev = "v${version}";
    hash = "sha256-x4B0vEUJkBIcuePkUG3/Y0cayN29n+jwXrG33QHV7NI=";
  };

  cargoHash = "sha256-/RmESSUTk7k+nVKLFDBm3AH9m8Tpd5fZhv0pxx3oKKA=";

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
