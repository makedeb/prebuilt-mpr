# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.54'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}" 'libappindicator1')
bullseye_depends=("${_base_depends[@]}" 'libayatana-appindicator1')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('b09aff50867682b007badd25044c0bcfd9ab99961c1befa834ca955e20872640909c8f540c0c9a6d7916ba42387123c981ad3cbccca756e670a96b3d46f22275')

package() {
    tar -xf 'data.tar.gz' -C "${pkgdir}"
    mkdir -p "${pkgdir}/DEBIAN"
    tar -xf 'control.tar.gz' -C "${pkgdir}/DEBIAN"
    rm "${pkgdir}/DEBIAN/control"
}
