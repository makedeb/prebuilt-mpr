# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=drone-cli
pkgver=1.7.0
pkgrel=1
pkgdesc='Command Line Tools for Drone CI'
arch=('any')
makedepends=('golang-go>=1.16')
conflicts=('drone-cli-bin')
license=('Apache-2.0')
url='https://docs.drone.io/cli'

source=("https://github.com/harness/drone-cli/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
	cd "${pkgname}-${pkgver}/"
	go build -trimpath -ldflags "-X main.version=${pkgver}" -o "${pkgname}" ./drone
}

package() {
	install -Dm 655 "${pkgname}-${pkgver}/${pkgname}" "${pkgdir}/usr/bin/drone"
}

# vim: set ts=4 sw=4 expandtab:
