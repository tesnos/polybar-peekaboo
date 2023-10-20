# Maintainer: tesnos6921 <ki@domain.com>
pkgname="polybar-peekaboo"
pkgver="0.1.0"
pkgrel=1
pkgdesc=""
arch=('x86_64')
url=""
license=('GPL')
groups=()
depends=('i3-wm' 'polybar')
makedepends=(cargo)
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=($pkgname-$pkgver.tar.gz)
noextract=()
md5sums=('794e7de47d5319da7bc3673a0b5dbadd') #autofill using updpkgsums

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}