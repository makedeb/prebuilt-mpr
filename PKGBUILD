# Maintainer: Hunter Wittenborn <hunter@hunterwittenborn.com>
# Contributor: hiddeninthesand <hiddeninthesand at pm dot me>
pkgbase=rustc
pkgname=(
    'rustc'
    'cargo'
    'libstd-rust-dev'
    'rust-all'
    'rust-clippy'
    'rust-doc'
    'rust-gdb'
    'rust-lldb'
    'rust-src'
    'rustfmt'
)
pkgver=1.77.0
pkgrel=1
pkgdesc='The Rust programming language toolchain'
arch=('any')
makedepends=(
    'g++>=5.1'
    'python3'
    'make>=3.81'
    'cmake>=3.13.4'
    'ninja-build'
    'curl'
    'git'
    'libssl-dev'
    'pkg-config'
)
url='https://www.rust-lang.org'
options=('!strip' '!lto')

source=(
    "https://static.rust-lang.org/dist/rustc-${pkgver}-src.tar.gz"
    'config.toml'
)
sha256sums=(
    'SKIP'
    'SKIP'
)

build() {
    cd "rustc-${pkgver}-src/"
    args=(
        'python3'
        'x.py'
        '--config'
        "${srcdir}/config.toml"
        'build'
        '-j'
        "$(nproc)"
    )

    # If we're building in a CI environment, `x.py` requires us to start builds at stage 2.
    if [[ "${TF_BUILD:+x}" == 'x' || "${GITHUB_ACTIONS:+x}" == 'x' ]]; then
        args+=('--stage' '2')
    fi

    "${args[@]}"
}

package_rustc() {
    cd "rustc-${pkgver}-src/"
    DESTDIR="${pkgdir}" python3 x.py --config "${srcdir}/config.toml" install -j "$(nproc)"
}

package_cargo() {
    pkgdesc='Transitional package - cargo -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_libstd-rust-dev() {
    pkgdesc='Transitional package - libstd-rust-dev -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-all() {
    pkgdesc='Transitional package - rust-all -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-clippy() {
    pkgdesc='Transitional package - rust-clippy -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-doc() {
    pkgdesc='Transitional package - rust-doc -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-gdb() {
    pkgdesc='Transitional package - rust-gdb -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-lldb() {
    pkgdesc='Transitional package - rust-lldb -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rust-src() {
    pkgdesc='Transitional package - rust-src -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

package_rustfmt() {
    pkgdesc='Transitional package - rustfmt -> rustc'
    depends=("rustc=${pkgver}-${pkgrel}")
}

# vim: set sw=4 expandtab:
