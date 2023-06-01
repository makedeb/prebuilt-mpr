# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
_pkgname=proget-cli
pkgname=pg
pkgver=0.1.0
pkgrel=2
pkgdesc='A CLI ProGet client'
arch=('any')
makedepends=('cargo')
license=('GPL-3.0')
url="https://github.com/hwittenborn/${_pkgname}"

source=("${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "${_pkgname}-${pkgver}/"
    cargo build --release
}

package() {
    cd "${_pkgname}-${pkgver}/"
    install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
