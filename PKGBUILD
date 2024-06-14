# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=rclone
pkgver=1.67.0
pkgrel=1
pkgdesc='rsync for cloud storage'
arch=('any')
depends=('libfuse-dev')
makedepends=('golang-go>=2:1.17')
license=('MIT')
url='https://rclone.org'

source=("https://github.com/rclone/rclone/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('4ecf2e99eb98c9bb678be5b0cd28550c4a2a2d63b5f2ed66962a4f4b9b36c402')

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
