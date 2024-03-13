# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=rclone
pkgver=1.66.0
pkgrel=1
pkgdesc='rsync for cloud storage'
arch=('any')
depends=('libfuse-dev')
makedepends=('golang-go>=2:1.17')
license=('MIT')
url='https://rclone.org'

source=("https://github.com/rclone/rclone/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('9249391867044a0fa4c5a948b46a03b320706b4d5c4d59db9d4aeff8d47cade2')

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
