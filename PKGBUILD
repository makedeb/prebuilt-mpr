# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.102'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}")
optdepends=('libappindicator1: Allow the app do display a menu in the system tray', 'libayatana-appindicator1: Allow the app to display a menu in the system tray')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('b8a875d03a013dc0deefc84be6e3b478d60ce8d11194a05e3764ca44493ac89d547ad51679c1f6f841672e0186b6c01bbf9113497fd7beebec6d3951e61aafd6')

package() {
    tar -xf 'control.tar.gz'
    tar -xf 'data.tar.gz' -C "${pkgdir}"

    postinst=src/postinst
}
