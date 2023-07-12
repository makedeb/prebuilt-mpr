# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=docker-compose
pkgver=2.19.1
pkgrel=1
pkgdesc='Define and run multi-container applications with Docker'
arch=('any')
license=('Apache-2.0')
makedepends=('golang-go>=1.17')
url='https://docs.docker.com/compose/'

source=("https://github.com/docker/compose/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "compose-${pkgver}/"
    go build -trimpath -o "${pkgname}" -ldflags="-X=github.com/docker/compose/v2/internal.Version=v${pkgver}" ./cmd
}

package() {
    cd "compose-${pkgver}/"
    install -Dm 755 "./${pkgname}" "${pkgdir}/usr/lib/docker/cli-plugins/${pkgname}"
    install -d "${pkgdir}/usr/bin/"
    ln -s "/usr/lib/docker/cli-plugins/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
