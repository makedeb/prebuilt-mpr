# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.85'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}")
optdepends=('libappindicator1: Allow the app do display a menu in the system tray', 'libayatana-appindicator1: Allow the app to display a menu in the system tray')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('8b2aef7a76e032c37beac4e12ed8e6d9c7db15c4d4d6d6433204fe72ae3ac551f7b2c5dbc10351ad1a95b6747ad4d2164bfbef1c9e70d9e4bff235754cc529d4')

package() {
    tar -xf 'control.tar.gz'
    tar -xf 'data.tar.gz' -C "${pkgdir}"

    postinst=src/postinst
}
