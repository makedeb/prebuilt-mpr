# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
_pkgname=appimagetool
pkgname=appimagetool-bin
pkgver=13
pkgrel=1
pkgdesc='Package desktop applications as AppImages that run on common Linux-based operating systems'
depends=('desktop-file-utils' 'zsync')
provides=("${_pkgname}=${pkgver}")
arch=('amd64')
license=('MIT')
url='https://appimage.org'

source=("https://github.com/AppImage/AppImageKit/releases/download/${pkgver}/${_pkgname}-x86_64.AppImage")
sha256sums=('df3baf5ca5facbecfc2f3fa6713c29ab9cefa8fd8c1eac5d283b79cab33e4acb')

build() {
    chmod +x "${_pkgname}-x86_64.AppImage"
    "./${_pkgname}-x86_64.AppImage" --appimage-extract
}

package() {
    mkdir -p "${pkgdir}/usr/"{bin,share}
    mv squashfs-root "${pkgdir}/usr/share/${pkgname}"

    ln -s "/usr/share/${pkgname}/usr/bin/${_pkgname}" "${pkgdir}/usr/bin/${_pkgname}"
}

# vim: set sw=4 expandtab:
