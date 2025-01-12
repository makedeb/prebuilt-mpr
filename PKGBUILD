# Maintainer: Evan Goode <mail@evangoo.de>
# Contributor: lordpipe <lordpipe@protonmail.com>
# Contributor: txtsd <mpr.makedeb@ihavea.quest>
# Contributor: Sefa Eyeoglu <conctact@scrumplex.net>
# Contributor: dada513 <dada513@protonmail.com>

pkgname=fjordlauncher
pkgver=9.2.0
pkgrel=1
pkgdesc='Prism Launcher fork with support for alternative auth servers'
arch=('i386' 'amd64' 'arm64' 'armhf' 'riscv64')
url='https://github.com/unmojang/FjordLauncher'
license=('GPL-3')
depends=(
  'libqt6core5compat6'
  'libqt6core6'
  'libqt6network6'
  'libqt6networkauth6'
  'libqt6svg6'
  'libqt6widgets6'
  'libqt6xml6'
  'qt6-image-formats-plugins'
)
makedepends=(
  'cmake'
  'extra-cmake-modules'
  'g++'
  'gcc'
  'git'
  'libgl1-mesa-dev'
  'libqt6core5compat6-dev'
  'openjdk-17-jdk'
  'qt6-base-dev'
  'qt6-networkauth-dev'
  'qtchooser'
  'scdoc'
  'zlib1g-dev'
)
optdepends=(
  'flite: narrator support'
  'java-runtime=17: support for Minecraft versions >= 1.17 and <= 1.20.4'
  'java-runtime=21: support for Minecraft versions >= 1.20.5'
  'java-runtime=8: support for Minecraft versions <= 1.16'
  'x11-xserver-utils: xrandr is needed to support Minecraft versions <= 1.12'
  's!gamemode: support for GameMode'
  's!mangohud: HUD overlay for FPS and temperatures'
)
source=(
  "https://github.com/unmojang/FjordLauncher/releases/download/${pkgver}/FjordLauncher-${pkgver}.tar.gz"
  'gcc-armv7-fix.patch'
  'copyright'
)
sha256sums=('86d2a8aac2efc88fd101bd95b1584a141e3f67a3f34b2c9647385b839f8083e4'
            '42394447d4b52c9329ff45f3c700c0eb2090a5803c5de010587d64294c37420f'
            'fefd606e959a9c6c8ba947a7d351b85f57f1d52b962bc3c674522b7a6fb48460')
postinst=postinst.sh

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
  case "${CARCH}" in
    amd64)
      CFLAGS+=" -march=x86-64 -fcf-protection"
      CXXFLAGS+=" -march=x86-64 -fcf-protection"
      ;;
    i386)
      CFLAGS+=" -march=i686"
      CXXFLAGS+=" -march=i686"
      ;;
    arm64)
      CFLAGS+=" -march=armv8-a"
      CXXFLAGS+=" -march=armv8-a"
      ;;
    armhf)
      CFLAGS+=" -march=armv7-a+fp"
      CXXFLAGS+=" -march=armv7-a+fp"
      ;;
    riscv64)
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
    patch --directory="FjordLauncher-${pkgver}" --forward --strip=1 --input="${srcdir}/gcc-armv7-fix.patch"
  else
    echo "GCC / ARMv7 fix is not needed for this architecture, skipping gcc-armv7-fix.patch"
  fi
}

build() {
  cd "${srcdir}/FjordLauncher-${pkgver}"
  cmake -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX="/usr" \
    -DLauncher_BUILD_PLATFORM="debian" \
    -DLauncher_APP_BINARY_NAME="${pkgname}" \
    -DLauncher_ENABLE_JAVA_DOWNLOADER=ON \
    -DENABLE_LTO=ON \
    -Bbuild -S.
  cmake --build build
}

check() {
  cd "${srcdir}/FjordLauncher-${pkgver}/build"
  ctest . -E Task  # Skip unreliable Task test
}

package() {
  cd "${srcdir}/FjordLauncher-${pkgver}/build"
  DESTDIR="${pkgdir}" cmake --install .
  mkdir -p "${pkgdir}/usr/share/doc/${pkgname}"
  mv "${pkgdir}/usr/share/mime/packages/modrinth-mrpack-mime.xml" \
     "${pkgdir}/usr/share/mime/packages/fjordlauncher-modrinth-mrpack-mime.xml"
  cp -v "${srcdir}/copyright" "${pkgdir}/usr/share/doc/${pkgname}/copyright"
}

# vim: set sw=2 expandtab:
