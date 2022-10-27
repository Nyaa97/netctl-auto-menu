_pkgname=netctl-auto-men
pkgname=${_pkgname}
pkgver=1
pkgrel=1
pkgdesc='A wifi menu for netctl'
arch=('x86_64')
url="https://github.com/Nyaa97/${_pkgname}"
license=('GPL')
depends=('dzen2' 'netctl' 'xorg-xprop')
makedepends=('git' 'rustup')
optdepends=()
provides=("${_pkgname}")
conflicts=()
source=("https://github.com/Nyaa97/${_pkgname}")
md5sums=('SKIP')

build() {
  cd "$srcdir/$_pkgname"
  cargo build
}

package() {
  installDir="$pkgdir/usr/bin"
  install -m755 "$srcdir/$_pkgname/target/$_pkgname" "$installDir/$_pkgname"
}
