# MSYS2 Packaging for OpenVAF

This repository now includes automated MSYS2 packaging for Windows users who prefer the MSYS2 environment.

## Automated Builds

The CI/CD pipeline automatically builds MSYS2 packages for:
- **MINGW64** environment
- **UCRT64** environment

These packages are built on every push and pull request, and the resulting `.pkg.tar.zst` files are uploaded as GitHub Actions artifacts.

## Package Contents

The MSYS2 package includes:
- **Binary**: `openvaf-r` - The main OpenVAF compiler
- **Documentation**: README, changelogs, and internals documentation
- **License**: GPL-3.0 license file
- **Header**: OSDI 0.4 header file for development

## Installation Locations

When installed via the MSYS2 package:
- Binary: `${MINGW_PREFIX}/bin/openvaf-r.exe`
- Docs: `${MINGW_PREFIX}/share/doc/openvaf/`
- License: `${MINGW_PREFIX}/share/licenses/openvaf/`
- Header: `${MINGW_PREFIX}/include/osdi_0_4.h`

## Building Locally

To build the MSYS2 package locally:

1. Set up MSYS2 with development tools:
   ```bash
   pacman -S base-devel git
   pacman -S mingw-w64-x86_64-rust mingw-w64-x86_64-clang mingw-w64-x86_64-llvm
   pacman -S mingw-w64-x86_64-openssl mingw-w64-x86_64-pkg-config
   ```

2. Clone the repository and build:
   ```bash
   git clone https://github.com/Kreijstal/OpenVAF.git
   cd OpenVAF
   makepkg --noconfirm --syncdeps
   ```

3. Install the package:
   ```bash
   pacman -U *.pkg.tar.zst
   ```

## Dependencies

The package depends on:
- `mingw-w64-x86_64-gcc-libs`
- `mingw-w64-x86_64-openssl`

Build dependencies include:
- `mingw-w64-x86_64-rust`
- `mingw-w64-x86_64-clang`
- `mingw-w64-x86_64-llvm`
- `mingw-w64-x86_64-cc`
- `mingw-w64-x86_64-pkg-config`