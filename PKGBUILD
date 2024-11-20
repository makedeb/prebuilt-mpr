# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=4 expandtab:
epoch=2
repology_pkgname=go
pkgname=golang-go-bin
pkgver=1.23.3
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
sha256sums_amd64=('a0afb9744c00648bafb1b90b4aba5bdb86f424f02f9275399ce0c20b93a2c3a8')
sha256sums_arm64=('1f7cbd7f668ea32a107ecd41b6488aaee1f5d77a66efd885b175494439d4e1ce')

extensions=('zipman')

package() {
  mkdir -p "${pkgdir}/usr/local/"
  cp -R "${srcdir}/go" "${pkgdir}/usr/local/go"
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/local/go/bin/go" "${pkgdir}/usr/bin/go"
  ln -s "/usr/local/go/bin/gofmt" "${pkgdir}/usr/bin/gofmt"
}
