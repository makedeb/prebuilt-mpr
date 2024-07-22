# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=just
pkgver=1.32.0
pkgrel=1
pkgdesc='Just a command runner'
arch=('any')
makedepends=('cargo>=1.63')
license=('CC0-1.0')
url='https://just.systems'

source=("https://github.com/casey/just/archive/refs/tags/${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --release
}

package() {
    cd "${pkgname}-${pkgver}/"
    install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    "target/release/${pkgname}" --completions bash | install -Dm 644 /dev/stdin "${pkgdir}/usr/share/bash-completion/completions/just"
    "target/release/${pkgname}" --man | install -Dm 644 /dev/stdin "${pkgdir}/usr/share/man/man1/${pkgname}.1"
}

# vim: set sw=4 expandtab:
