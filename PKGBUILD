# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=bat
pkgver=0.22.1
pkgrel=1
pkgdesc='A cat(1) clone with wings'
arch=('any')
makedepends=('rustc>=1.51' 'cargo')
conflicts=('bacula-console-qt')
license=('Apache-2.0')
url='https://github.com/sharkdp/bat'

source=("${pkgname}-${pkgver}::git+${url}/#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
    true
    cd "${pkgname}-${pkgver}/"
    cargo build --release --all-features
}

package() {
    cd "${pkgname}-${pkgver}/target/release"
    install -Dm 755 ./bat "${pkgdir}/usr/bin/bat"

    # We can't guarantee which folder Cargo decides to place build assets
    # inside of, so we need to search for it.
    cd build/
    find ./ -name 'bat.1' -exec install -Dm 644 '{}' "${pkgdir}/usr/share/man/man1/bat.1" \;
    find ./ -name 'bat.bash' -exec install -Dm 644 '{}' "${pkgdir}/usr/share/bash-completion/completions/bat" \;
}

# vim: set sw=4 expandtab:
