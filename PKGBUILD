# Maintainer: Sony Stinks <sonystinks@protonmail.com>
# Contributor: dada513 <dada513@protonmail.com>
# Contributor: Sefa Eyeoglu <contact@scrumplex.net>
# Contributor: Lenny McLennington <lennymclennington@protonmail.com>
# Contributor: Yellow <yellow@example.com>
# Contributor: Elijah Gregg <lovetocode999@tilde.team>
# Contributor: Miko <mikoxyzzz@gmail.com>
# Contributor: Cheru Berhanu <aur attt cheru doot dev>

pkgname=polymc
pkgver=6.1
pkgrel=0.1
pkgdesc="Minecraft launcher with ability to manage multiple instances."
arch=('amd64')
url="https://github.com/PolyMC/PolyMC"
license=('GPL3')
depends=('libqt5svg5' 'qt5-image-formats-plugins' 'libqt5xml5' 'libqt5core5a' 'libqt5network5' 'libqt5gui5' 'libqt5charts5-dev')
provides=('polymc')
conflicts=('polymc')
makedepends=('scdoc' 'extra-cmake-modules' 'cmake' 'git' 'openjdk-17-jdk' 'zlib1g-dev' 'libgl1-mesa-dev' 'qtbase5-dev' 'qtchooser' 'qt5-qmake' 'qtbase5-dev-tools' 'gcc' 'g++')
optdepends=('java-runtime=8: support for Minecraft versions < 1.17'
            'java-runtime=17: support for Minecraft versions >= 1.17')	    
source=("https://github.com/PolyMC/PolyMC/releases/download/$pkgver/PolyMC-$pkgver.tar.gz")

sha256sums=("16d62604f7e4aed0a9a31876b860e5054ca12e1c81fe47e74324eb1edec9d8d0")

build() {

  cmake -DCMAKE_BUILD_TYPE= \
    -DCMAKE_INSTALL_PREFIX="/usr" \
    -DLauncher_QT_VERSION_MAJOR=5 \
    -Bbuild -SPolyMC-$pkgver
  cmake --build build
}

check() {
  cd "${srcdir}/build"
  ctest .
}

package() {
  cd "build"
  DESTDIR="$pkgdir" cmake --install .
}
