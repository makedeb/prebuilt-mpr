# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=4 expandtab:
epoch=2
repology_pkgname=go
pkgname=golang-go-bin
pkgver=1.20.6
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
sha256sums_arm64=('4e15ab37556e979181a1a1cc60f6d796932223a0f5351d7c83768b356f84429b')
sha256sums_amd64=('b945ae2bb5db01a0fb4786afde64e6fbab50b67f6fa0eb6cfa4924f16a7ff1eb')

extensions=('zipman')

package() {
  mkdir -p "${pkgdir}/usr/local/"
  cp -R "${srcdir}/go" "${pkgdir}/usr/local/go"
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/local/go/bin/go" "${pkgdir}/usr/bin/go"
  ln -s "/usr/local/go/bin/gofmt" "${pkgdir}/usr/bin/gofmt"
}
