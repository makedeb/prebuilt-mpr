# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
_pkgname='cli'
pkgname=npm
pkgver=8.18.0
pkgrel=2
pkgdesc='A JavaScript package manager'
arch=('all')
depends=('nodejs')
makedepends=('git' 'python3-setuptools')
url='https://docs.npmjs.com/cli'

source=("${_pkgname}-${pkgver}::git+https://github.com/npm/${_pkgname}/#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
    cd "${_pkgname}-${pkgver}/"
    NODE_PATH=/usr/lib/node_modules make
}

package() {
    cd "${_pkgname}-${pkgver}/"
    node . install -g --prefix="${pkgdir}/usr" "$(node . pack |& tail -1)"
    node . completion | install -Dm 644 /dev/stdin "${pkgdir}/usr/share/bash-completion/completions/${pkgname}"
}

# vim: set sw=4 expandtab:
