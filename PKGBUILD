# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=rclone
pkgver=1.62.2
pkgrel=2
pkgdesc='rsync for cloud storage'
arch=('any')
depends=('libfuse-dev')
makedepends=('golang-go>=2:1.17')
license=('MIT')
url='https://rclone.org'

source=("https://github.com/rclone/rclone/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('6741c81ae5b5cb48a04055f280f6e220ed4b35d26fe43f59510d0f7740044748')

build () {
    cd "${pkgname}-${pkgver}/"
    go build -trimpath -ldflags "-s -X github.com/rclone/rclone/fs.Version=v${pkgver}" -tags cmount
}

package() {
    cd "${pkgname}-${pkgver}/"
    install -Dm 755 "./${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
    "./${pkgname}" completion bash | install -Dm 644 /dev/stdin "${pkgdir}/usr/share/bash-completion/completions/${pkgname}"
}

# vim: set sw=4 expandtab:
