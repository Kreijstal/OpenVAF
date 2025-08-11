# Maintainer: OpenVAF Team
_realname=openvaf
pkgbase=mingw-w64-${_realname}
pkgname=("${MINGW_PACKAGE_PREFIX}-${_realname}")
pkgver=23.5.0
pkgrel=1
pkgdesc="OpenVAF - Verilog-A compiler that generates OSDI-compliant dynamic libraries (mingw-w64)"
arch=('any')
mingw_arch=('mingw64' 'ucrt64')
url="https://github.com/Kreijstal/OpenVAF"
license=('GPL3')
depends=("${MINGW_PACKAGE_PREFIX}-gcc-libs"
         "${MINGW_PACKAGE_PREFIX}-openssl")
makedepends=("${MINGW_PACKAGE_PREFIX}-rust"
             "${MINGW_PACKAGE_PREFIX}-clang"
             "${MINGW_PACKAGE_PREFIX}-llvm"
             "${MINGW_PACKAGE_PREFIX}-cc"
             "${MINGW_PACKAGE_PREFIX}-pkg-config")
source=()
sha256sums=()

build() {
    cd "${srcdir}/.."
    
    # Set environment variables for cross-compilation
    export RUSTFLAGS="-C target-feature=+crt-static"
    export PKG_CONFIG_ALLOW_CROSS=1
    
    # Build the project in release mode
    cargo build --release --verbose
}

package() {
    cd "${srcdir}/.."
    
    # Install the main binary
    if [[ "${MINGW_CHOST}" == *"-w64-mingw32" ]]; then
        install -Dm755 "target/release/openvaf-r.exe" "${pkgdir}${MINGW_PREFIX}/bin/openvaf-r.exe"
    else
        install -Dm755 "target/release/openvaf-r" "${pkgdir}${MINGW_PREFIX}/bin/openvaf-r"
    fi
    
    # Install documentation
    install -Dm644 README.md "${pkgdir}${MINGW_PREFIX}/share/doc/${_realname}/README.md"
    install -Dm644 LICENSE "${pkgdir}${MINGW_PREFIX}/share/licenses/${_realname}/LICENSE"
    install -Dm644 CHANGELOG_OSDI.md "${pkgdir}${MINGW_PREFIX}/share/doc/${_realname}/CHANGELOG_OSDI.md"
    install -Dm644 CHANGELOG_VERILOGAE.md "${pkgdir}${MINGW_PREFIX}/share/doc/${_realname}/CHANGELOG_VERILOGAE.md"
    install -Dm644 internals.md "${pkgdir}${MINGW_PREFIX}/share/doc/${_realname}/internals.md"
    
    # Install OSDI header files
    install -Dm644 "openvaf/osdi/header/osdi_0_4.h" "${pkgdir}${MINGW_PREFIX}/include/osdi_0_4.h"
}

check() {
    cd "${srcdir}/.."
    
    # Run tests
    cargo test --release
}