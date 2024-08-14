# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.64'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}")
optdepends=('libappindicator1: Allow the app do display a menu in the system tray', 'libayatana-appindicator1: Allow the app to display a menu in the system tray')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('0f7dc7e2bfd0809fca616748ab0168ac77a2a9cbbb66bae823f8e3f9b29e853b96b7601ed843c50dd684ae9802366515ba945e3a68bc15a94881ed75c1ce7047')

package() {
    tar -xf 'control.tar.gz'
    tar -xf 'data.tar.gz' -C "${pkgdir}"

    postinst=src/postinst
}
