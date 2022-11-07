{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    alsaLib
    fontconfig
    libxkbcommon
    pkg-config
    udev
    udev
    vulkan-loader
    xorg.libxcb
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath [
      alsaLib
      fontconfig
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
