# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=matrix-commander-rs
pkgver=0.1.24
pkgrel=3
pkgdesc='Simple but convenient CLI-based Matrix client app'
arch=('any')
depends=('libssl3')
makedepends=('cargo' 'libssl-dev' 'pkg-config')
license=('GPL-3.0')
url='https://github.com/8go/matrix-commander-rs'

source=("${pkgname}-${pkgver}::git+${url}")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --bin "${pkgname}" --release
}

package() {
    cd "${pkgname}-${pkgver}"
    install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
