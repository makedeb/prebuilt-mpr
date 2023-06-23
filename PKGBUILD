# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
pkgname=cloudflare-warp
pkgver=2023.3.470
pkgrel=1
pkgdesc='Cloudflare Warp Client'
arch=('amd64')
url='https://developers.cloudflare.com/cloudflare-one/connections/connect-devices/warp/'

xenial_source=("https://pkg.cloudflareclient.com/pool/xenial/main/c/cloudflare-warp/${pkgname}_${pkgver}-1_amd64.deb")
bionic_source=("https://pkg.cloudflareclient.com/pool/bionic/main/c/cloudflare-warp/${pkgname}_${pkgver}-1_amd64.deb")
focal_source=("https://pkg.cloudflareclient.com/pool/focal/main/c/cloudflare-warp/${pkgname}_${pkgver}-1_amd64.deb")
source=("https://pkg.cloudflareclient.com/pool/jammy/main/c/cloudflare-warp/${pkgname}_${pkgver}-1_amd64.deb")

xenial_sha256sums=('5a65ce64c32d8f24bb75bde08f5d0a0d78aa178b87691851f27f81673f52c529')
bionic_sha256sums=('934d70a81fcc593702b99d99259001d245fc50b042450012c76dca3ad38382be')
focal_sha256sums=('642b85c831f8e4821e2abf14c519e0c0537c0c36577236b45e80836845b0fa9c')
sha256sums=('0189e68ac1102858840768912627c4a916ddcc0c8171fad4d084de52890a2da8')

package() {
    if ! [[ -d data ]]; then
        mkdir data
    fi
    tar xf data.tar.gz -C data

    cd data
    for path in *; do
        cp "${path}" "${pkgdir}/" -r
    done
}

# vim: set sw=4 expandtab:
