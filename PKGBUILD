# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=maturin
pkgver=0.13.2
pkgrel=1
pkgdesc='Build and publish crates with pyo3, rust-cpython and cffi bindings'
arch=('any')
makedepends=('cargo')
license=('Apache-2.0' 'MIT')
url='https://maturin.rs/'

source=("https://github.com/PyO3/maturin/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --release --all-features
}

package() {
    cd "${pkgname}-${pkgver}/"
    install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
