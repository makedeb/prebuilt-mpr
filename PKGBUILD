# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=pg
pkgver=0.1.1
pkgrel=1
pkgdesc='A CLI ProGet client'
arch=('any')
makedepends=('cargo')
license=('GPL-3.0')
url='https://github.com/hwittenborn/proget-cli'

source=("${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --release
}

package() {
    cd "${pkgname}-${pkgver}/"
    install -Dm 755 target/release/pg "${pkgdir}/usr/bin/pg"
}

# vim: set sw=4 expandtab:
