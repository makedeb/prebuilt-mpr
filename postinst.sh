#!/bin/bash
cat << EOF
*==================== PrismLauncher for Debian and Ubuntu ====================*
 Welcome to PrismLauncher!

 The following optional peer dependencies are available for integration:

 - GameMode - Optimise Linux system performance on demand
   apt install gamemode
 - MangoHUD - Overlay for monitoring FPS, temperatures, CPU/GPU load and more
   apt install mangohud

 Note that alternative distributions of PrismLauncher are also available for
 Debian and Ubuntu via the following package managers:

 - Flatpak: https://flathub.org/apps/org.prismlauncher.PrismLauncher
    Provides sandboxing, but requires slightly more disk space.
    Works on all recent Debian and Ubuntu versions.
 - Nix: https://github.com/PrismLauncher/PrismLauncher/blob/develop/flake.nix
    Works on all recent Debian and Ubuntu versions.
 - AppImage and Portable: https://prismlauncher.org/download/linux/
    Not guaranteed to work on all recent Debian and Ubuntu versions.
 - Pi-Apps: https://pi-apps.io/wiki/getting-started/apps-list/
    Optimized for older Raspberry Pis running Raspberry Pi OS
    Not guaranteed to work on all recent Debian and Ubuntu versions.

 This package is intended for use with legacy KDE 5.x distributions such as
 Debian 12 or Ubuntu 24.04. If this package is not properly adapting to your
 KDE theme, consider using the \`prismlauncher\` package instead, or use
 the Flatpak.

 Need help?
 <3 PrismLauncher Discord: https://discord.gg/ArX2nafFz2
 <3 PrismLauncher Matrix: https://matrix.to/\#/\#prismlauncher:matrix.org
 <3 PrismLauncher subreddit: https://www.reddit.com/r/PrismLauncher/

 Bug reports: https://github.com/PrismLauncher/PrismLauncher/issues
 Packaging bug reports: https://github.com/makedeb/prebuilt-mpr/issues
*=============================================================================*
EOF
