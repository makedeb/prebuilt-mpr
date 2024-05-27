# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=2 expandtab:
pkgname=helix-bin
pkgver=24.03
pkgrel=2
pkgdesc='A post-modern modal text editor.'
arch=('amd64')
provides=('hx')
conflicts=('helix')
license=('Mozilla Public License Version 2.0')
url='https://helix-editor.com/'
extensions=('zipman')

source=(
  "https://github.com/helix-editor/helix/releases/download/${pkgver}/helix-${pkgver}-x86_64-linux.tar.xz"
  "hx"
)
sha256sums=('b15290807fedf4907abc700a408c2a4e74662a3008a46224bf18aeca38379bfc'
            '55164f8c97c79ee919057356d3dff03c33c987cfabb6de65c822f226d6a7d500')

package() {
  mkdir -p "$pkgdir/usr/lib/helix/"
  cp -r "$srcdir/helix-${pkgver}-x86_64-linux/runtime/" "$pkgdir/usr/lib/helix/"
  cp -r "$srcdir/helix-${pkgver}-x86_64-linux/hx" "$pkgdir/usr/lib/helix/hx"
  install -Dm 0644 "$srcdir/helix-${pkgver}-x86_64-linux/LICENSE" "${pkgdir}/usr/share/licenses/helix/LICENSE"
  install -D "$srcdir/hx" "$pkgdir/usr/bin/hx"
}
