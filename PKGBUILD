# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=minecraft-launcher
pkgver=1.0.1221
pkgrel=3
pkgdesc='Official Minecraft Launcher'
arch=(
    'amd64'
)
depends=(
    'p!dpkg>=1.14.0'
    'p!wget|curl'
    'p!ca-certificates'
    'default-jre'
    'libasound2>=1.0.23'
    'libatk-bridge2.0-0>=2.5.3'
    'libatk1.0-0>=2.2.0'
    'libatspi2.0-0>=2.9.90'
    'libc6>=2.16'
    'libcairo2>=1.6.0'
    'libcups2>=1.4.0'
    'libdbus-1-3>=1.5.12'
    'libdrm2>=2.4.38'
    'libexpat1>=2.0.1'
    'libgbm1>=8.1~0'
    'libfontconfig1>=2.8.0'
    'libgcc1>=1:4.1.1'
    'libgdk-pixbuf2.0-0>=2.22.0'
    'libglib2.0-0>=2.39.4'
    'libgtk-3-0>=3.18.9'
    'libnspr4>=2:4.9-2~'
    'libnss3>=2:3.22'
    'libpango1.0-0>=1.14.0|libpango-1.0-0>=1.14.0'
    'libpangocairo-1.0-0>=1.14.0'
    'libstdc++6>=4.8.0'
    'libx11-6>=2:1.4.99.1'
    'libxcomposite1>=1:0.3-1'
    'libxcursor1>1.1.2'
    'libxdamage1>=1:1.1'
    'libxext6'
    'libxfixes3'
    'libxi6>=2:1.2.99.4'
    'libxrandr2>=2:1.2.99.3'
    'libxrender1'
    'libxss1'
    'libxtst6'
    'libx11-xcb1'
    'libxcb-dri3-0'
    'libxcb1>=1.9.2'
    'libbz2-1.0'
    'lsb-base>=4.1'
    'xdg-utils>=1.0.2'
    'wget'
    'libcurl3|libcurl4'
    'libuuid1'
    'gnome-keyring'
)
url='https://minecraft.net'

source=(
    "https://launcher.mojang.com/download/linux/x86_64/minecraft-launcher_${pkgver}.tar.gz"
    'https://launcher.mojang.com/download/minecraft-launcher.svg'
    'minecraft-launcher.desktop'
)
sha256sums=(
    'SKIP'
    'SKIP'
    'SKIP'
)

package() {
    install -Dm 644 "${pkgname}.svg" "${pkgdir}/usr/share/icons/hicolor/symbolic/apps/${pkgname}.svg"
    install -Dm 644 "${pkgname}.desktop" "${pkgdir}/usr/share/applications/${pkgname}.desktop"
    install -Dm 755 "${pkgname}/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
