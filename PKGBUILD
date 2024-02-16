# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=4 expandtab:
epoch=2
repology_pkgname=go
pkgname=golang-go-bin
pkgver=1.22.0
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
sha256sums_amd64=('f6c8a87aa03b92c4b0bf3d558e28ea03006eb29db78917daec5cfb6ec1046265')
sha256sums_arm64=('6a63fef0e050146f275bf02a0896badfe77c11b6f05499bb647e7bd613a45a10')

extensions=('zipman')

package() {
  mkdir -p "${pkgdir}/usr/local/"
  cp -R "${srcdir}/go" "${pkgdir}/usr/local/go"
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/local/go/bin/go" "${pkgdir}/usr/bin/go"
  ln -s "/usr/local/go/bin/gofmt" "${pkgdir}/usr/bin/gofmt"
}
