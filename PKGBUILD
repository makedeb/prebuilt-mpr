# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=woeusb-ng
pkgver=0.2.10
pkgrel=3
pkgdesc='Simple tool that enables you to create your own USB stick Windows installer'
arch=('any')
depends=(
    'dosfstools'
    'grub-pc-bin'
    'grub2-common'
    'ntfs-3g'
    'p7zip-full'
    'parted'
    'python3-setuptools'
    'python3-termcolor'
    'python3-wxgtk4.0'
)
optdepends=(
    'r!python3-termcolor'
)
conflicts=(
    'woeusb'
)
provides=('woeusb')
license=('GPL3')
url='https://github.com/WoeUSB/WoeUSB-ng'

source=("${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

package() {
    cd "WoeUSB-ng-${pkgver}/"
    python3 setup.py install --root="${pkgdir}" --install-layout deb
    
    install -d "${pkgdir}/usr/bin/"
    mkdir -p "${pkgdir}/usr/bin"
    install -Dm 755 WoeUSB/{woeusb,woeusbgui} -t "${pkgdir}/usr/bin/"
    install -Dm 644 WoeUSB/data/icon.ico "${pkgdir}/usr/share/icons/WoeUSB-ng/icon.ico"
    install -Dm 644 miscellaneous/WoeUSB-ng.desktop "${pkgdir}/usr/share/applications/WoeUSB-ng.desktop"
    install -Dm 644 miscellaneous/com.github.woeusb.woeusb-ng.policy "${pkgdir}/usr/share/polkit-1/actions/com.github.woeusb.woeusb-ng.policy"

}

# vim: set sw=4 expandtab:
