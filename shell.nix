
let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    wrapGAppsHook4
    gst_all_1.gst-plugins-base 
  ];

  buildInputs = with pkgs; [
    librsvg
    webkitgtk_4_1
  ];

  shellHook = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH" # Needed on Wayland to report the correct display scale
  '';
}

