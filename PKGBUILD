# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=4 expandtab:
epoch=2
repology_pkgname=go
pkgname=golang-go-bin
pkgver=1.24.1
pkgrel=1
pkgdesc='The Go programming language'
arch=(amd64 arm64)
depends=(tar)
extensions=('zipman')
provides=("golang-go=2:${pkgver}")
conflicts=(golang-go)
license=(BSD3)
url='https://github.com/golang/go'

source_arm64=("https://go.dev/dl/go${pkgver}.linux-arm64.tar.gz")
source_amd64=("https://go.dev/dl/go${pkgver}.linux-amd64.tar.gz")
sha256sums_amd64=('cb2396bae64183cdccf81a9a6df0aea3bce9511fc21469fb89a0c00470088073')
sha256sums_arm64=('8df5750ffc0281017fb6070fba450f5d22b600a02081dceef47966ffaf36a3af')

extensions=('zipman')

package() {
  mkdir -p "${pkgdir}/usr/local/"
  cp -R "${srcdir}/go" "${pkgdir}/usr/local/go"
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/local/go/bin/go" "${pkgdir}/usr/bin/go"
  ln -s "/usr/local/go/bin/gofmt" "${pkgdir}/usr/bin/gofmt"
}
