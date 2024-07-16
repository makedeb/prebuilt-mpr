# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=element-desktop-bin
pkgver=1.11.71
pkgrel=1
pkgdesc="A feature-rich client for Matrix.org"
arch=('amd64')
url="https://element.io/"
license=('Apache-2.0')
depends=('libgtk-3-0' 'libnotify4' 'libnss3' 'libxss1' 'libxtst6' 'xdg-utils' 'libatspi2.0-0' 'libuuid1' 'libsecret-1-0' 'libsqlcipher0')
optdepends=('libappindicator3-1')
conflicts=('riot-desktop<1.7.0' 'riot-web<1.7.0')
replaces=('riot-desktop<1.7.0' 'riot-web<1.7.0')

source=("https://packages.element.io/debian/pool/main/e/element-desktop/element-desktop_${pkgver}_amd64.deb")
sha256sums=('SKIP')

package() {
    tar -xf "${srcdir}/data.tar.xz" -C "${pkgdir}"
    mkdir -p "${pkgdir}/usr/bin/"
    ln -s /opt/Element/element-desktop "${pkgdir}/usr/bin/element-desktop"
}
