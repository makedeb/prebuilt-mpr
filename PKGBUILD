# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.46'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}" 'libappindicator1')
bullseye_depends=("${_base_depends[@]}" 'libayatana-appindicator1')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('8cb9a16b2e0cfcb2bd086b35a6639b3ecfd8480b77dc472a2121da3d454f431b44a2a9e115ff97fe5206cf4e31decf2eb351cb07671afc4c24cd65e5748f7db1')

package() {
    tar -xf 'data.tar.gz' -C "${pkgdir}"
    mkdir -p "${pkgdir}/DEBIAN"
    tar -xf 'control.tar.gz' -C "${pkgdir}/DEBIAN"
    rm "${pkgdir}/DEBIAN/control"
}
