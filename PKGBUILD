pkgname=netctl-auto-menu
pkgver=1
pkgrel=1
pkgdesc='A wifi menu for netctl'
arch=('x86_64')
url="https://github.com/Nyaa97/${_pkgname}"
license=('GPL')
depends=('dzen2' 'netctl' 'xorg-xprop')
makedepends=('git' 'rustup')
optdepends=()
provides=("${pkgname}")
conflicts=()
source=("git+https://github.com/Nyaa97/${pkgname}")
md5sums=('SKIP')

prepare() {
  ln -s "$srcdir/../$pkgname" "$srcdir"
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  install -Dm755 "$srcdir/$pkgname/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
