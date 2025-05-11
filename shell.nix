{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer

    # --- Zależności systemowe dla Bevy i jego zależności ---
    pkgs.pkg-config  # Kluczowy brakujący element!
    pkgs.alsa-lib    # Starsza nazwa, czasami używana zamiennie lub jako alias, lepiej mieć obie na wszelki wypadek
                     # Ważniejsze są pakiety deweloperskie - często zawarte w głównym pakiecie alsaLib,
                     # ale czasami jako osobny alsaLib.dev lub libasound2-dev (w Nix to zazwyczaj jest w głównym)

    # Zależności dla Wayland/X11 (jeśli Bevy ich wymaga do budowania backendu okienkowania)
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
    pkgs.xorg.libXinerama # Może być potrzebne
    pkgs.vulkan-loader    # Dla Vulkan (domyślny backend graficzny Bevy na Linuksie)
    pkgs.libGL            # Dla OpenGL, jeśli używany jako fallback

    # Dodatkowe, które mogą być potrzebne dla Wayland
    pkgs.wayland
    pkgs.libinput
    pkgs.mesa             # Sterowniki graficzne

    # Możliwe, że będziesz potrzebować też:
    # pkgs.udev
    # pkgs.libdrm
    # pkgs.libxshmfence
  ];

  # Możesz też ustawić zmienne środowiskowe, jeśli są potrzebne, np.
  # RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  # LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}