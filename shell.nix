{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  hardeningDisable = [
    "fortify"
  ];

  buildInputs = with pkgs; [
    alsaLib
    fontconfig
    libxkbcommon
    pkg-config
    udev
    udev
    vulkan-loader
    xorg.libxcb
    spirv-tools
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath [
      alsaLib
      fontconfig
      freetype
      gcc-unwrapped.lib
      libxkbcommon
      udev
      vulkan-loader
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      xorg.libxcb
    ]}"
  '';
}
