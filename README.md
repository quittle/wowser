# Wowser

This is a side-project writen to learn Rust and therefore likely contains numerous issues, lacks in performance, and may have security or availability risks.

As a general rule, it avoids using 3rd-party dependencies where possible to provide more opportunities to learn.

## Building

This project is built with Rust. To get started, install [`rustup`](https://rustup.rs) and build with `cargo`.

### Windows via WSL

[Install VcXsrv](https://sourceforge.net/projects/vcxsrv) so the client can draw.

### Windows without WSL

Download and [install Windows C++ build tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019). Make sure to install the `Windows*SDK` in addition to the `MSVC C++ build tools`.

### Ubuntu (Including within a WSL distribution)
```
sudo apt install cmake libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev mesa-common-dev pkg-config libssl-dev libgl1-mesa-dev
```