#!/bin/bash
cat << EOF
*==================== PrismLauncher for Debian and Ubuntu ====================*
 Welcome to PrismLauncher! You will need to install the Java version
 appropriate for the Minecraft versions you wish to play:

 - Minecraft classic to Minecraft 1.16:
   # apt install java-runtime=8
 - Minecraft 1.17 to 1.20.4:
   # apt install java-runtime=17
 - Minecraft 1.20.5 and above:
   # apt install java-runtime=21

 Depending on the support cycles for your distribution, you may
 need to install a distribution of Java from the Adoptium OpenJDK
 archives. See https://adoptium.net/installation/linux/ for more details

 The following optional peer dependencies are available for integration:

 - CMU Flite - Speech synthesis engine that Minecraft uses for the narrator
   # apt install flite
 - GameMode - Optimise Linux system performance on demand
   # apt install gamemode
 - MangoHUD - Overlay for monitoring FPS, temperatures, CPU/GPU load and more
   # apt install mangohud

 Note that alternative distributions of PrismLauncher are also available for
 Debian and Ubuntu via the following package managers:

 - Flatpak: https://flathub.org/apps/org.prismlauncher.PrismLauncher
    Provides sandboxing and ships Java versions and Qt versions automatically.
    Works on all recent Debian and Ubuntu versions.
 - Nix: https://github.com/PrismLauncher/PrismLauncher/blob/develop/flake.nix
    Works on all recent Debian and Ubuntu versions.
 - AppImage and Portable: https://prismlauncher.org/download/linux/
    Not guaranteed to work on all recent Debian and Ubuntu versions.
 - Pi-Apps: https://pi-apps.io/wiki/getting-started/apps-list/
    Optimized for older Raspberry Pis running Raspberry Pi OS
    Not guaranteed to work on all recent Debian and Ubuntu versions.

 This package is intended for use with KDE Plasma 6.x distributions such
 as KDE Neon 6 and future Kubuntu versions. If this package is not properly
 adapting to your KDE theme, consider using the \`prismlauncher-qt5\`
 package instead, or use the Flatpak.

 Need help?

 PrismLauncher Discord: https://discord.gg/ArX2nafFz2
 PrismLauncher Matrix: https://matrix.to/\#/\#prismlauncher:matrix.org
 PrismLauncher subreddit: https://www.reddit.com/r/PrismLauncher/

 Bug reports: https://github.com/PrismLauncher/PrismLauncher/issues
 Packaging bug reports: https://github.com/makedeb/prebuilt-mpr/issues
*=============================================================================*
EOF
