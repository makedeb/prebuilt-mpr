# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=rustup-init
pkgver=1.26.0
pkgrel=2
pkgdesc='The installer for rustup'
arch=('all')
depends=('curl')

source=("https://github.com/rust-lang/rustup/archive/refs/tags/${pkgver}.tar.gz")
sha256sums=('6f20ff98f2f1dbde6886f8d133fe0d7aed24bc76c670ea1fca18eb33baadd808')

package() {
    cd "rustup-${pkgver}"
    install -Dm 755 "${pkgname}.sh" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
