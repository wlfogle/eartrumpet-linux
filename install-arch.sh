#!/usr/bin/env bash
set -euo pipefail

# Manjaro/Arch setup helper for this repository (Rust)
TIMEOUT="${TIMEOUT:-25s}"

if ! command -v pacman >/dev/null 2>&1; then
  echo "This script is intended for Manjaro/Arch (pacman). Exiting."
  exit 1
fi

sudo -v || { echo "sudo privileges are required"; exit 1; }

sudo pacman -S --needed --noconfirm base-devel git curl wget jq ripgrep fd pkgconf cmake make unzip
sudo pacman -S --needed --noconfirm rustup cargo
if ! command -v rustc >/dev/null 2>&1; then
  rustup default stable || true
fi

echo "[OK] Manjaro setup complete for $(basename "$PWD")."
