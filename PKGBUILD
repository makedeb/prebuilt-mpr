# Maintainer: Sefa Eyeoglu <conctact@scrumplex.net>
# Contributor: dada513 <dada513@protonmail.com>
# Contributor: lordpipe <lordpipe@protonmail.com>

pkgname=prismlauncher
pkgver=7.2
pkgrel=5
pkgdesc='Minecraft launcher with ability to manage multiple instances.'
arch=('i386' 'amd64' 'arm64' 'armhf' 'riscv64')
url='https://prismlauncher.org'
license=('GPL3')
depends=('libqt5svg5' 'qt5-image-formats-plugins' 'libqt5xml5' 'libqt5core5a' 'libqt5network5' 'libqt5gui5')
makedepends=('scdoc' 'extra-cmake-modules' 'cmake' 'git' 'openjdk-17-jdk' 'zlib1g-dev' 'libgl1-mesa-dev' 'qtbase5-dev' 'qtchooser' 'qt5-qmake' 'qtbase5-dev-tools' 'gcc' 'g++')
optdepends=('java-runtime=17: support for Minecraft versions >= 1.17'
            's!java-runtime=8: support for Minecraft versions <= 1.16'
            's!gamemode: support for GameMode'
            's!flite: narrator support'
            's!x11-xserver-utils: xrandr is needed to support Minecraft versions <= 1.12')
source=("https://github.com/PrismLauncher/PrismLauncher/releases/download/$pkgver/PrismLauncher-$pkgver.tar.gz"
        'gcc-armv7-fix.patch')
sha256sums=('5733b55c4532286813a6fb7de2f3a38e6f6db743a251919c8b646d32a84514b4'
            '42394447d4b52c9329ff45f3c700c0eb2090a5803c5de010587d64294c37420f')

# allow for ARM support
#TODO: makedeb's hard-coding for x86-64 has been fixed in a future makedeb version
#TODO: these 8 lines make this script match the behavior of future makedeb. When it releases, remove this
CARCH="$(dpkg --print-architecture)"
CHOST="$(uname -m)-pc-linux-gnu"
CFLAGS=${CFLAGS/-march=x86-64/}
CXXFLAGS=${CXXFLAGS/-march=x86-64/}
CFLAGS=${CFLAGS/-mtune=generic/}
CXXFLAGS=${CXXFLAGS/-mtune=generic/}
CFLAGS=${CFLAGS/-fcf-protection/}
CXXFLAGS=${CXXFLAGS/-fcf-protection/}

# if the user hasn't specified a tuning/architecture, specify our own minimal defaults to cover the earliest CPUs
if [[ ${CFLAGS} != *"-mtune"* && ${CFLAGS} != *"-march"* ]]; then
    case "$(uname -m)" in
        x86_64)
            CFLAGS+=" -march=x86-64 -mtune=generic -fcf-protection"
            CXXFLAGS+=" -march=x86-64 -mtune=generic -fcf-protection"
            ;;
        i686)
            CFLAGS+=" -march=i686 -mtune=generic"
            CXXFLAGS+=" -march=i686 -mtune=generic"
            ;;
        aarch64*|armv8*|armv9*|arm64*)
            CFLAGS+=" -march=armv8-a -mtune=generic"
            CXXFLAGS+=" -march=armv8-a -mtune=generic"
            ;;
        arm*)
            CFLAGS+=" -march=armv7-a+fp"
            CXXFLAGS+=" -march=armv7-a+fp"
            ;;
        riscv64*)
            CFLAGS+=" -march=rv64imafdc"
            CXXFLAGS+=" -march=rv64imafdc"
            ;;
    esac
fi

prepare() {
    # workaround https://gcc.gnu.org/bugzilla/show_bug.cgi?id=64860
    # more info: https://github.com/PrismLauncher/PrismLauncher/issues/128
    if [[ "$(uname -m)" = armv7* ]]; then
        echo "GCC / ARMv7 fix is needed for this architecture, applying gcc-armv7-fix.patch"
        patch --directory="PrismLauncher-$pkgver" --forward --strip=1 --input="${srcdir}/gcc-armv7-fix.patch"
    else
        echo "GCC / ARMv7 fix is not needed for this architecture, skipping gcc-armv7-fix.patch"
    fi
}

build() {
    cd "${srcdir}/PrismLauncher-$pkgver"
    cmake -DCMAKE_BUILD_TYPE=None \
          -DCMAKE_INSTALL_PREFIX="/usr" \
          -DLauncher_BUILD_PLATFORM="debian" \
          -DLauncher_APP_BINARY_NAME="${pkgname}" \
          -DLauncher_QT_VERSION_MAJOR=5 \
          -DENABLE_LTO=ON \
          -Bbuild -S.
    cmake --build build
}

check() {
    cd "${srcdir}/PrismLauncher-$pkgver/build"
    ctest . -E Task  # Skip unreliable Task test
}

package() {
    cd "${srcdir}/PrismLauncher-$pkgver/build"
    DESTDIR="$pkgdir" cmake --install .
}
