# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.43'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('x86_64' 'amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}" 'libappindicator1')
bullseye_depends=("${_base_depends[@]}" 'libayatana-appindicator1')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('3ace6309c3d3d84bf196e3b02f02070c691badf803d366e357fb4b45d39081ffa9586e9ac27eafcebdc2dafa8123ddf014bb62d3ab54f87248744c7dbb8dfa3b')

package() {
    tar -xf 'data.tar.gz' -C "${pkgdir}"
    mkdir -p "${pkgdir}/DEBIAN"
    tar -xf 'control.tar.gz' -C "${pkgdir}/DEBIAN"
    rm "${pkgdir}/DEBIAN/control"
}
