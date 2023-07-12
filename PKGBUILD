# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=rclone
pkgver=1.63.0
pkgrel=1
pkgdesc='rsync for cloud storage'
arch=('any')
depends=('libfuse-dev')
makedepends=('golang-go>=2:1.17')
license=('MIT')
url='https://rclone.org'

source=("https://github.com/rclone/rclone/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('755af528052f946e8d41a3e96e5dbf8f03ecfe398f9d0fdeb7ca1a59208a75db')

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
