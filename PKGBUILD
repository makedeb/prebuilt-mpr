# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname='cargo-appimage'
pkgver=2.0.0
pkgrel=1
pkgdesc='Converts your crate into an AppImage'
arch=('any')
depends=('appimagetool' 'awk' 'cargo' 'file')
license=('GPL3')

source=("https://github.com/StratusFearMe21/${pkgname}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('77fbbd87d5a7ccc565bb01e2ecc3735dfbaa8fa3e70831728f6f808d90ddfe63')

prepare() {
    cd "${pkgname}-${pkgver}/"
    # cargo-appimage looks in `${CARGO_HOME}` for the 'cargo-appimage-runner'
    # binary. This patch makes it look in '/usr/bin/' instead.
    sed -i 's|"bin/cargo-appimage-runner"|"/usr/bin/cargo-appimage-runner"|' src/main.rs
}

build() {
    cd "${pkgname}-${pkgver}/"
    cargo build --release
}

package() {
    cd "${pkgname}-${pkgver}/"
    for bin in "${pkgname}" "${pkgname}-runner"; do
        install -Dm 755 "target/release/${bin}" "${pkgdir}/usr/bin/${bin}"
    done
}

# vim: set sw=4 expandtab:
