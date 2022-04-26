# Wowser [![Build Status](https://github.com/quittle/wowser/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/quittle/wowser/actions/workflows/rust.yml?query=branch%3Amain)

This is a side-project writen to learn Rust and therefore likely contains numerous issues, lacks in performance, and may have security or availability risks.

As a general rule, it avoids using 3rd-party dependencies where possible to provide more opportunities to learn.

## Building

This project is built with Rust. To get started, install [`rustup`](https://rustup.rs) and build with `cargo`.

### Windows via WSL

[Install VcXsrv](https://sourceforge.net/projects/vcxsrv) so the client can draw.

#### Easy Way

Run this from within WSL

```bash
source scripts/windows_x11
```

#### Manual Way

Start `XLaunch` with **Native opengl** disabled and **Disable access control** checked or run this from within WSL.

```bash
wslview config.xlaunch
```

Run this from a WSL terminal before running something that opens a window to connect to the Windows X11 server.

```bash
export DISPLAY=$(route.exe print | grep 0.0.0.0 | head -1 | awk '{print $4}'):0.0
```

### Windows without WSL

Download and [install Windows C++ build tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019). Make sure to install the `Windows*SDK` in addition to the `MSVC C++ build tools`.

### Ubuntu (Including within a WSL distribution)

```
sudo apt install clang cmake libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev mesa-common-dev pkg-config libssl-dev libgl1-mesa-dev
```

## Guidelines for Development

1. When parsing protocols, normalize data as early as possible to minimize the chance of bugs and reduce the cognitive load of keep track of normalizations
1. When normalizing strings, prefer lowercase over uppercase where possible.
1. When running UI tests, the windows are headless by default to help stabilize them. To show them in situations like debugging, set the environment variable `WOWSER_HEADLESS` to `false` when invoking.
