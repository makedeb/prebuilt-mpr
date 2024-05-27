# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=parse-changelog
pkgver=0.5.0
pkgrel=1
pkgdesc='Simple changelog parser, written in Rust'
arch=('any')
makedepends=('cargo' 'rustc>=1.51')
license=('Apache-2.0' 'MIT')
url='https://docs.rs/parse-changelog'

source=("https://github.com/taiki-e/parse-changelog/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --release
}

package() {
    cd "${pkgname}-${pkgver}/"
    install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
