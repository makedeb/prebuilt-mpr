# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.87'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}")
optdepends=('libappindicator1: Allow the app do display a menu in the system tray', 'libayatana-appindicator1: Allow the app to display a menu in the system tray')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('92fc61abe44f20a5b2a5bd1db594d594617798edf7f4e91fc5d7dd0d5ebc56fe9d841f6609d7e2c32093103255290773d910d9de1829ec1338e6083b48425042')

package() {
    tar -xf 'control.tar.gz'
    tar -xf 'data.tar.gz' -C "${pkgdir}"

    postinst=src/postinst
}
