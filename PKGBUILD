# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=google-chrome-stable
pkgver=126.0.6478.114
pkgrel=1
pkgdesc='The web browser from Google'
arch=('amd64')
depends=(
    'p!dpkg>=1.14.0'
    'ca-certificates'
    'fonts-liberation'
    'libasound2>=1.0.16'
    'libatk-bridge2.0-0>=2.5.3'
    'libatk1.0-0>=2.2.0'
    'libatspi2.0-0>=2.9.90'
    'libc6>=2.17'
    'libcairo2>=1.6.0'
    'libcups2>=1.6.0'
    'libcurl3-gnutls|libcurl3-nss|libcurl4|libcurl3'
    'libdbus-1-3>=1.5.12'
    'libdrm2>=2.4.38'
    'libexpat1>=2.0.1'
    'libgbm1>=8.1~0'
    'libgcc1>=1:3.0'
    'libglib2.0-0>=2.39.4'
    'libgtk-3-0>=3.9.10|libgtk-4-1'
    'libnspr4>=2:4.9-2~'
    'libnss3>=2:3.26'
    'libpango-1.0-0>=1.14.0'
    'libx11-6>=2:1.4.99.1'
    'libxcb1>=1.9.2'
    'libxcomposite1>=1:0.4.4-1'
    'libxdamage1>=1:1.1'
    'libxext6'
    'libxfixes3'
    'libxkbcommon0>=0.4.1'
    'libxrandr2'
    'wget'
    'xdg-utils>=1.0.2'
)
optdepends=(
    'r!libu2f-udev'
    'r!libvulkan1'
)
provides=(
    'www-browser'
)

url='https://www.google.com/chrome'
options=('!strip')

source=("${pkgname}.deb::https://dl.google.com/linux/chrome/deb/pool/main/g/${pkgname}/${pkgname}_${pkgver}-1_amd64.deb")
sha256sums=('086aa8e979d4c2774e8ce0cec72f93086c435db033e04c2464c3a5ed92c5b007')

package() {
    tar xf control.tar.xz
    tar xf data.tar.xz -C "${pkgdir}"

    # Disable Google's APT repository.
    install -d "${pkgdir}/etc/default"
    touch "${pkgdir}/etc/default/google-chrome"

    postinst=src/postinst
    postrm=src/postrm
    prerm=src/prerm
}

# vim: set sw=4 expandtab:
