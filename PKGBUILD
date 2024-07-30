# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.60'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}")
optdepends=('libappindicator1: Allow the app do display a menu in the system tray', 'libayatana-appindicator1: Allow the app to display a menu in the system tray')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('94485fe1b28cbc43971b17bd8a25e0716361e64c57e67956855ffdfafe245a3911f2bccbd403230224daa71771add36b5caf990934b2917d89335b65854e0972')

package() {
    tar -xf 'control.tar.gz'
    tar -xf 'data.tar.gz' -C "${pkgdir}"

    postinst=src/postinst
}
