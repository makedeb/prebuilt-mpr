# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.45'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('x86_64' 'amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}" 'libappindicator1')
bullseye_depends=("${_base_depends[@]}" 'libayatana-appindicator1')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('4c44b1d8214acadd934ba55ba8dd9ff0a5ef43ec3dd43e6455b9e679620a297f5ebf8768657d1a776e7c484266c765a9c4eb771f056ebb6f352ee95c0f031554')

package() {
    tar -xf 'data.tar.gz' -C "${pkgdir}"
    mkdir -p "${pkgdir}/DEBIAN"
    tar -xf 'control.tar.gz' -C "${pkgdir}/DEBIAN"
    rm "${pkgdir}/DEBIAN/control"
}
