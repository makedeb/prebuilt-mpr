# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=neovim
pkgver=0.9.1
pkgrel=1
pkgdesc='Vim-fork focused on extensibility and usability'
arch=('any')
makedepends=(
    'ninja-build'
    'gettext'
    'libtool'
    'libtool-bin'
    'autoconf'
    'automake'
    'cmake'
    'g++'
    'pkg-config'
    'unzip'
    'curl'
    'doxygen'
)
conflicts=('neovim-runtime')
postinst="${pkgname}.postinst"
prerm="${pkgname}.prerm"
license=('Apache-2.0')
url='https://neovim.io/'

source=("${pkgname}-${pkgver}::git+https://github.com/neovim/neovim/#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
    cd "${pkgname}-${pkgver}/"
    make CMAKE_BUILD_TYPE='Release'
}

package() {
    cd "${pkgname}-${pkgver}/"
    DESTDIR="${pkgdir}" make CMAKE_INSTALL_PREFIX='/usr' install
}

# vim: set sw=4 expandtab
