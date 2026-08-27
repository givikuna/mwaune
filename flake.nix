{
  description = "mwaune - book mgmt soft";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        naersk' = pkgs.callPackage naersk { };

        tauriApp = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            gtk3
            webkitgtk_4_1
            libsoup_3
            glib
            gdk-pixbuf
            librsvg

            nodejs_latest
            nodePackages_latest.npm
          ];
          buildInputs = with pkgs; [
            openssl
            dbus
            atk
            gtk3
            glib
            cairo
            pango
            gdk-pixbuf
            webkitgtk_4_1
            librsvg
            libxkbcommon
          ];

          preBuild = ''
            npm install
            npm run build
          '';

          TAURI_SKIP_BUILD = "1";
          TAURI_WEBVIEW = "webkit2gtk"; # linux
        };
      in
      {
        packages.default = tauriApp;
        apps.default = {
          type = "app";
          program = "${tauriApp}/bin/mwaune";
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            nodejs_latest
            pkg-config
            gtk3
            webkitgtk_4_1
            libsoup_3
            glib
            gdk-pixbuf
            librsvg
            openssl
            dbus
            libxkbcommon
          ];

          shellHook = ''
            echo "hi"
          '';
        };
      }
    );
}
