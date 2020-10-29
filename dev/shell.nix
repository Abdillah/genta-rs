let
  nixpkgs = import <nixpkgs> {};
  libs = with nixpkgs.pkgs; [
    # Compiler requirements
    zlib
    llvmPackages.libclang

    glib
    tracker.out
  ];
  ldpath = with nixpkgs.pkgs; nixpkgs.lib.makeLibraryPath libs;
  pkgcfgpath = with nixpkgs.pkgs; nixpkgs.lib.makeSearchPathOutput "lib" "lib/pkgconfig" [
    tracker.dev
    glib.dev
    clang
  ];

  self = with nixpkgs; nixpkgs.stdenv.mkDerivation rec {
    name = "gtracker-${version}-builder0";
    version = "rev-4c34dd";

    nativeBuildInputs = [
        clang
        pkgconfig

        # Runtime
        gdb
    ];
    buildInputs = libs;

    doCheck = false;

    shellHooks = ''
      LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${ldpath}";
      export LD_LIBRARY_PATH;

      PKG_CONFIG_PATH="$PKG_CONFIG_PATH:${pkgcfgpath}";
      export PKG_CONFIG_PATH;

      PATH=$PATH:~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin
      export XDG_DATA_DIRS=$XDG_DATA_DIRS:$GSETTINGS_SCHEMAS_PATH
    '';
  };
in self
