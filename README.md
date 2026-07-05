# AcreetionOS Terminal

A modern terminal emulator for the AcreetionOS ecosystem — built with Rust for performance, cross-platform compatibility, and Wayland/X11 support.

## About

AcreetionOS Terminal is designed to be fast, lightweight, and responsive. It comes with sensible defaults and extensive configuration options, providing a smooth terminal experience for developers, sysadmins, and everyday users.

### Features

- GPU-accelerated rendering via OpenGL
- Cross-platform (Linux, BSD, macOS, Windows)
- Wayland and X11 support
- TOML-based configuration
- Vi-mode cursor movement
- Scrollback buffer
- Customizable color schemes
- Font ligatures support
- True color support

## Installation

### Arch Linux / AcreetionOS

AcreetionOS Terminal is available on the AUR:

```bash
paru -S acreetionos-terminal
# or
yay -S acreetionos-terminal
```

### Prebuilt Binaries

Precompiled binaries are available from the [releases page](https://github.com/spivanatalie64/acreetionos-terminal/releases).

### Building from Source

```bash
git clone https://github.com/spivanatalie64/acreetionos-terminal.git
cd acreetionos-terminal
cargo build --release
```

#### Requirements

- Rust (latest stable)
- OpenGL ES 2.0+
- Linux: `cmake`, `pkg-config`, `libfontconfig`, `libxcb`, `libxkbcommon`

## Configuration

Configuration is done via `~/.config/acreetionos-terminal/acreetionos-terminal.toml`.  
See the [wiki](https://acreetionos.org/wiki.conf) for full configuration documentation.

## Documentation & Wiki

Full documentation, including configuration guides, troubleshooting, and contribution guidelines:

➡️ **https://acreetionos.org/wiki.conf**

## Project Information

- **Status:** Active development
- **License:** Apache 2.0
- **Language:** Rust
- **Maintainer:** Natalie Spiva (@spivanatalie64)

## Contributing

Contributions are welcome! Please check the [wiki](https://acreetionos.org/wiki.conf) for guidelines.

## Contact

- **Issues:** https://github.com/spivanatalie64/acreetionos-terminal/issues
- **Website:** https://acreetionos.org
