# Maintainer: Menci <huanghaorui301@gmail.com>

_repo=fatedier/frp

pkgname=frp
pkgver=0.43.0
pkgrel=1
pkgdesc="A fast reverse proxy to help you expose a local server behind a NAT or firewall to the internet"
arch=('any')
url="https://github.com/${_repo}"
license=('Apache')
makedepends=('wget' 'git')
source=(
    "frpc@.service"
    "frps@.service"
)
sha512sums=(
    "b472ca11412ac9688a6fbfa70617d565fa070647182b05348a2a50ee0379a7cb880ef7fd4c48da038e0659cf976f9c36c9498e00014b10e28c70dee30dba38c4"
    "f3aad121ff84fa9475e9122d6a61103dddc038567be161a8b190ed695adf62d9d894a4484c3c669d6d018550ffc8019e13e1fcd26b7ca585420327296a2ef958"
)

pkgver() {
    curl https://api.github.com/repos/${_repo}/releases/latest | sed -nE 's/^  "tag_name": "v(.+)",$/\1/p'
}

_goarch() {
    DPKG_ARCH="$(dpkg --print-architecture)"
    if [[ "$DPKG_ARCH" == "i386" ]]; then
        echo "386"
    else
        echo "$DPKG_ARCH"
    fi
}

build() {
    cd "${srcdir}"

    RELEASE_FILENAME="frp_${pkgver}_linux_$(_goarch)"
    wget "https://github.com/${_repo}/releases/download/v${pkgver}/${RELEASE_FILENAME}.tar.gz" -O "${RELEASE_FILENAME}.tar.gz"
    tar xvf "${RELEASE_FILENAME}.tar.gz"
    mv "${RELEASE_FILENAME}" "${pkgname}"
}

package() {
    cd "${srcdir}"

    mkdir -p "$pkgdir/etc/$pkgname"
    install -Dm 755 "$pkgname/$pkgname"{c,s} -t "$pkgdir/usr/bin"
    install -Dm 644 "$pkgname/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    for svc in "${pkgname}"{c,s}"@.service"; do
        install -Dm 644 "$svc" -t "$pkgdir/usr/lib/systemd/system/"
    done

    # Add install scripts
    prerm="src/prerm"
    echo '[ "$1" = "upgrade" ] && exit 0' > "${srcdir}/prerm"
    echo "systemctl stop 'frpc@*'" >> "${srcdir}/prerm"
    echo "systemctl disable frpc@" >> "${srcdir}/prerm"
    echo "systemctl stop 'frps@*'" >> "${srcdir}/prerm"
    echo "systemctl disable frps@" >> "${srcdir}/prerm"
}
