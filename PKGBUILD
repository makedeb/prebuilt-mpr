# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=doppler
pkgver=3.60.1
pkgrel=1
pkgdesc='The official Doppler CLI for managing your secrets'
arch=('any')
license=('Apache-2.0')
url='https://github.com/DopplerHQ/cli'
makedepends=('golang-go')

source=("${pkgname}-${pkgver}.tar.gz::${url}/archive/refs/tags/${pkgver}.tar.gz")
sha256sums=('57c8e1c84e496f9c04bd090901bf9b757a8b7f3f23c2b673c4a7b9c153509add')

build() {
    cd "cli-${pkgver}/"
    go build -o "build/${pkgname}" -ldflags="-X github.com/DopplerHQ/cli/pkg/version.ProgramVersion=v${pkgver}"
}

package() {
    cd "cli-${pkgver}/"
    "build/${pkgname}" completion bash | install -Dm 644 /dev/stdin "${pkgdir}/usr/share/bash-completion/completions/${pkgname}"
    install -Dm 755 "build/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
