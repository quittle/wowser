# Wowser [![Build Status](https://github.com/quittle/wowser/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/quittle/wowser/actions/workflows/rust.yml?query=branch%3Amain)

This is a side-project writen to learn Rust and therefore likely contains numerous issues, lacks in performance, and may have security or availability risks.

As a general rule, it avoids using 3rd-party dependencies where possible to provide more opportunities to learn.

## Building

This project is built with Rust. To get started, install [`rustup`](https://rustup.rs) and build with `cargo`.

### Windows via WSL

[Install VcXsrv](https://sourceforge.net/projects/vcxsrv) so the client can draw.

Start `XLaunch` with **Native opengl** disabled and **Disable access control** checked.

Run this from a WSL terminal before starting to connect to the Windows X11 server.
```bash
export DISPLAY="$(cat /etc/resolv.conf | grep -oE '([0-9]{1,}\.){3}[0-9]{1,}'):0.0"
```

### Windows without WSL

Download and [install Windows C++ build tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019). Make sure to install the `Windows*SDK` in addition to the `MSVC C++ build tools`.

### Ubuntu (Including within a WSL distribution)
```
sudo apt install clang cmake libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev mesa-common-dev pkg-config libssl-dev libgl1-mesa-dev
```
