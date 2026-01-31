#!/bin/bash
# Bevy-Rust Development Environment Setup
# Run this script INSIDE the distrobox container

set -e

echo "=== Bevy-Rust Development Environment Setup ==="
echo "Installing dependencies for Bevy development..."
echo ""

# Update system
echo "Updating system packages..."
sudo dnf update -y

# 1. ADD RPM FUSION REPOSITORIES (Required for H.264/MP4 support)
echo "Enabling RPM Fusion repositories for FFmpeg and codecs..."
sudo dnf install -y \
  https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm \
  https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm

# Install Rust development tools and Bevy dependencies
echo "Installing development tools and libraries..."
sudo dnf install -y \
  gcc \
  gcc-c++ \
  make \
  cmake \
  pkg-config \
  openssl-devel \
  alsa-lib-devel \
  libudev-devel \
  libX11-devel \
  libXcursor-devel \
  libXrandr-devel \
  libXi-devel \
  mesa-libGL-devel \
  vulkan-loader \
  vulkan-headers \
  vulkan-validation-layers \
  wayland-devel \
  libxkbcommon-devel \
  libxkbcommon-x11-devel \
  libxkbcommon-x11 \
  ffmpeg \
  ffmpeg-devel \
  libva-utils \
  vdpauinfo

echo ""
echo "Installing Rust toolchain..."

# Install Rust using rustup
if ! command -v rustup &> /dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
  echo "Rust installed successfully!"
else
  echo "Rust already installed, updating..."
  rustup update stable
fi

# Ensure stable toolchain is default
rustup default stable

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "FFmpeg version: $(ffmpeg -version | head -n 1)"
echo ""
echo "Your Bevy development environment is ready!"
echo ""
echo "Next steps:"
echo "  1. Navigate to your project directory: cd ~/Code/bevy-rust"
echo "  2. Create a new project: cargo new my-bevy-game"
echo "  3. Use FFmpeg to stitch renders: ffmpeg -r 30 -i out/%05d.png -c:v libx264 -pix_fmt yuv420p out.mp4"
echo ""
