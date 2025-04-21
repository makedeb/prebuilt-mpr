# Maintainer: Sony Stinks <sonystinks@protonmail.com>
# Contributor: dada513 <dada513@protonmail.com>
# Contributor: Sefa Eyeoglu <contact@scrumplex.net>
# Contributor: Lenny McLennington <lennymclennington@protonmail.com>
# Contributor: Yellow <yellow@example.com>
# Contributor: Elijah Gregg <lovetocode999@tilde.team>
# Contributor: Miko <mikoxyzzz@gmail.com>
# Contributor: Cheru Berhanu <aur attt cheru doot dev>

pkgname=polymc
pkgver=7.0
pkgrel=1
pkgdesc="Minecraft launcher with ability to manage multiple instances."
arch=('amd64')
url="https://github.com/PolyMC/PolyMC"
license=('GPL3')
depends=('hicolor-icon-theme' 'openjdk-21-jre' 'libgl-dev' 'qt6-base-dev' 'qt6-5compat-dev' 
         'qt6-svg-dev' 'qt6-image-formats-plugins' 'qt6-charts-dev' 'libqt6charts6' 
         'libqt6core5compat6' 'libqt6core6t64' 'libqt6gui6t64' 'libqt6network6t64' 
         'libqt6networkauth6' 'libqt6svg6' 'libqt6xml6t64' 'libquazip1-qt6-dev' 'zlib1g-dev' 
         'libgl-dev' 'libglvnd-dev')
provides=('polymc')
conflicts=('polymc')
makedepends=('scdoc' 'extra-cmake-modules' 'cmake' 'git' 'openjdk-21-jdk' 'qtchooser' 
             'qmake6' 'qmake6-bin' 'qt6-tools-dev-tools' 'qt6-tools-dev' 'gcc' 'g++')
optdepends=('glfw: to use system GLFW libraries'
            'openal: to use system OpenAL libraries'
            'visualvm: Profiling support'
            'xorg-xrandr: for older minecraft versions'
            'java-runtime=8: support for Minecraft versions < 1.17'
            'java-runtime=17: support for Minecraft versions >= 1.17'
            'java-runtime=21: support for Minecraft versions >= 1.20')

source=("https://github.com/PolyMC/PolyMC/releases/download/$pkgver/PolyMC-$pkgver.tar.gz")

sha256sums=('e08e9a25f87db7da422351d044b330e4b1a568f3adabc04c388dc9e4f60c4701')

build() {

  cmake -DCMAKE_BUILD_TYPE= 'None'\
    -DCMAKE_INSTALL_PREFIX="/usr" \
    -DLauncher_BUILD_PLATFORM='debianlinux' \
    -DLauncher_QT_VERSION_MAJOR=6 \
    -U_FORTIFY_SOURCE \
    -D_FORTIFY_SOURCE=2 \
    -Bbuild -SPolyMC-$pkgver \
    -Wno-dev

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
