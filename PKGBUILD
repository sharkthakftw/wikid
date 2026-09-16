# Maintainer: sharkthakftw <sharkthakftw@gmail.com>
pkgname=wikid
pkgver=3.2.2
pkgrel=1
pkgdesc="feature-rich terminal wikipedia client"
arch=('x86_64' 'aarch64')
url="https://github.com/sharkthakftw/wikid"
license=('MIT')
depends=('gcc-libs' 'glibc')
optdepends=(
  'mpv: spoken article audio playback (recommended)'
  'ffmpeg: alternative audio playback backend (ffplay)'
  'vlc: alternative audio playback backend (cvlc)'
)
makedepends=('cargo')
options=('!lto')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('46764561f74f04df61271830422d548bb97215b6f05e21c4796812c336c9b611')

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 completions/wikid.fish "$pkgdir/usr/share/fish/vendor_completions.d/wikid.fish"
}
