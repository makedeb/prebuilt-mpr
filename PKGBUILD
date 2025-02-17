# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname='code-bin'
pkgver='1.97.2'
pkgrel='1'
pkgdesc="Code editing. Redefined."
arch=('amd64')
depends=('libnss3>=2:3.26' 'gnupg' 'apt' 'libxkbfile1' 'libsecret-1-0' 'libgtk-3-0>=3.10.0' 'libxss1' 'libgbm1')
provides=('visual-studio-code')
_base_url='code.visualstudio.com'
url="https://${_base_url}"

source=("code-${pkgver}.deb::https://update.${_base_url}/${pkgver}/linux-deb-x64/stable")
sha256sums=('SKIP')

package() {
  msg2 "Extracting data.tar.xz..."
  tar -xf data.tar.xz -C "${pkgdir}"

  msg2 "Setting up symlink to /usr/bin/code..."
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/share/code/bin/code" "${pkgdir}/usr/bin/code"
}
