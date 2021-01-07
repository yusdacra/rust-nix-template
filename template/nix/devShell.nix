{ common }:
with common; with pkgs;
mkShell {
  nativeBuildInputs =
    [ git nixpkgs-fmt cargo rustc ]
    ++ crateDeps.nativeBuildInputs;
  buildInputs = crateDeps.buildInputs;
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}"
  '';
}
