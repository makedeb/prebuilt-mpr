# Maintainer: Erwan BOTHUAN NIEL <dev dot erwanbo at caramail dot fr>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
# Contributor: zocker_160 <zocker1600 at posteo dot net>
# Contributor: Hunter Wittenborn <hunter@hunterwittenborn.com>

pkgname='discord'
pkgver='0.0.57'
pkgrel='1'
pkgdesc="Chat for Communities and Friends"
arch=('amd64')
_base_depends=('libc6' 'libasound2' 'libatomic1' 'libnotify4' 'libnspr4' 'libnss3' 'libstdc++6' 'libxss1' 'libxtst6')
depends=("${_base_depends[@]}" 'libappindicator1')
bullseye_depends=("${_base_depends[@]}" 'libayatana-appindicator1')
url="https://discord.com"
license=('custom')
source=("${pkgname}::https://dl.discordapp.net/apps/linux/${pkgver}/discord-${pkgver}.deb")
b2sums=('9821f4b2d95f5f9621857031e9d4957ee5c13145fce63b9b6e194f9f669cb99fa3e2ec200648adf2b45e4e45de4fa54bebbc1b45ee5d405a59e9121b8ea59964')

package() {
    tar -xf 'data.tar.gz' -C "${pkgdir}"
    mkdir -p "${pkgdir}/DEBIAN"
    tar -xf 'control.tar.gz' -C "${pkgdir}/DEBIAN"
    rm "${pkgdir}/DEBIAN/control"
}
