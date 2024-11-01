with import <nixpkgs> {};
mkShell {
    name = "hacklet";
    buildInputs = with pkgs; [
      ruby
      gem
      libftdi
      libftdi1

    ];
    nativeBuildInputs = [
      pkg-config
    ];
    shellHook = ''
      export LD_LIBRARY_PATH="${libftdi}/lib:${libftdi1}/lib"

      export PATH="$XDG_HOME_DIR/.gem/ruby/3.3.0/bin:$PATH"
    '';
}
