{pkgs, ...}: {
  languages.rust.enable = true;

  processes.build.exec = "cargo run";

  enterShell = ''
    rustc --version
  '';
}
