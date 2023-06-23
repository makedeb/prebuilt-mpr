# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=cloudflared
pkgver=2023.6.1
pkgrel=1
arch=('any')
pkgdesc='Cloudflare Tunnel client'
makedepends=('golang-go')
license=('Apache-2.0')
url='https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide'

source=("https://github.com/cloudflare/cloudflared/archive/refs/tags/${pkgver}.tar.gz")
sha256sums=('7f7509bb364f107541dc810410b763721c39cdfab85799080ccae96d1c4a9cff')

build() {
    cd "${pkgname}-${pkgver}/"

    local build_time="$(date -d "@${SOURCE_DATE_EPOCH}" '+%Y%m%d-%H:%M:%S')"
    go build -buildmode=pie \
        -ldflags "-compressdwarf=false
            -X main.Version=${pkgver}
            -X main.BuildTime=${build_time}
            -X github.com/cloudflare/cloudflared/cmd/cloudflared.updater.BuiltForPackageManager=MPR" \
        -o "build/${pkgname}" \
        "./cmd/${pkgname}"
}

package() {
    cd "${pkgname}-${pkgver}"
    ls
    install -Dm 755 "build/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

# vim: set sw=4 expandtab:
