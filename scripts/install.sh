#!/usr/bin/env sh
# Install the latest solidspec release binary for Linux or macOS — no Rust
# toolchain required. Downloads a prebuilt binary from GitHub Releases
# (published by .github/workflows/release.yml for every `vX.Y.Z` tag).
#
#   curl -fsSL https://raw.githubusercontent.com/jyjeanne/solidspec/master/scripts/install.sh | sh
#
# Install location defaults to $HOME/.local/bin; override with INSTALL_DIR.
# Windows: download solidspec-x86_64-pc-windows-msvc.zip from
# https://github.com/jyjeanne/solidspec/releases/latest and add it to PATH
# manually (see README.md).
set -eu

REPO="jyjeanne/solidspec"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)
    case "$arch" in
      x86_64) target="x86_64-unknown-linux-gnu" ;;
      *)
        echo "error: no prebuilt solidspec binary for Linux/$arch." >&2
        echo "Build from source instead: see README.md's 'Build from source' section." >&2
        exit 1
        ;;
    esac
    ext="tar.gz"
    ;;
  Darwin)
    case "$arch" in
      arm64) target="aarch64-apple-darwin" ;;
      x86_64) target="x86_64-apple-darwin" ;;
      *)
        echo "error: no prebuilt solidspec binary for macOS/$arch." >&2
        exit 1
        ;;
    esac
    ext="tar.gz"
    ;;
  *)
    echo "error: this script supports Linux and macOS only." >&2
    echo "Windows: download solidspec-x86_64-pc-windows-msvc.zip from" >&2
    echo "  https://github.com/${REPO}/releases/latest" >&2
    exit 1
    ;;
esac

asset="solidspec-${target}.${ext}"
url="https://github.com/${REPO}/releases/latest/download/${asset}"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Downloading ${asset}..."
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$url" -o "$tmp_dir/$asset"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$url" -O "$tmp_dir/$asset"
else
  echo "error: need curl or wget to download." >&2
  exit 1
fi

tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"

mkdir -p "$INSTALL_DIR"
cp "$tmp_dir/solidspec-${target}/solidspec" "$INSTALL_DIR/solidspec"
chmod +x "$INSTALL_DIR/solidspec"

echo "Installed solidspec to ${INSTALL_DIR}/solidspec"
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo
    echo "${INSTALL_DIR} is not on your PATH. Add it, e.g.:"
    echo "  echo 'export PATH=\"\$PATH:${INSTALL_DIR}\"' >> ~/.bashrc && source ~/.bashrc"
    ;;
esac

"$INSTALL_DIR/solidspec" --version
