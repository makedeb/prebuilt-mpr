# Maintainer: Jonathan Apodaca <jrapodaca@gmail.com>
# vim: set sw=4 expandtab:
epoch=2
repology_pkgname=go
pkgname=golang-go-bin
pkgver=1.22.1
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
sha256sums_amd64=('aab8e15785c997ae20f9c88422ee35d962c4562212bb0f879d052a35c8307c7f')
sha256sums_arm64=('e56685a245b6a0c592fc4a55f0b7803af5b3f827aaa29feab1f40e491acf35b8')

extensions=('zipman')

package() {
  mkdir -p "${pkgdir}/usr/local/"
  cp -R "${srcdir}/go" "${pkgdir}/usr/local/go"
  mkdir -p "${pkgdir}/usr/bin/"
  ln -s "/usr/local/go/bin/go" "${pkgdir}/usr/bin/go"
  ln -s "/usr/local/go/bin/gofmt" "${pkgdir}/usr/bin/gofmt"
}
