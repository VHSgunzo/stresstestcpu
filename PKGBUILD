# Maintainer: VHSgunzo <vhsgunzo.github.io>
pkgname='stresstestcpu'
pkgver='0.0.1'
pkgrel='1'
pkgdesc='Simple CPU stress-test in Rust'
arch=("x86_64")
url='https://github.com/VHSgunzo/stresstestcpu'
provides=("${pkgname}")
conflicts=("${pkgname}")
source=("${pkgname}::https://github.com/VHSgunzo/${pkgname}/releases/download/v${pkgver}/${pkgname}")
sha256sums=('SKIP')

package() { install -Dm755 ${pkgname} "$pkgdir/usr/bin/${pkgname}" ; }
