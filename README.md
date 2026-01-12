# Envy TUI

![](https://github.com/tassiovirginio/envy-tui/blob/main/sample.png?raw=true)

A Terminal User Interface (TUI) manager for [EnvyControl](https://github.com/bayasdev/envycontrol) - Easy GPU switching for Nvidia Optimus laptops under Linux.



![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## Features

- **Modern TUI** - Beautiful terminal interface built with Ratatui
- **Mode Switching** - Switch between Integrated, Hybrid, and Nvidia modes
- **Advanced Options** - Configure RTD3, Coolbits, and ForceCompositionPipeline
- **Visual Feedback** - Color-coded modes and clear status indicators
- **Keyboard Navigation** - Vim-style keybindings for efficient control

## Prerequisites

- [EnvyControl](https://github.com/bayasdev/envycontrol) installed on your system
- Rust toolchain (for building from source)

## Installation

### Arch Linux (AUR)

```bash
yay -S envy-tui-bin
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/envy-tui.git
cd envy-tui

# Build and install
cargo install --path .
```

## Usage

```bash
# Run the TUI
envy-tui
```

### Keybindings

| Key | Action |
|-----|--------|
| `↑`/`↓` or `j`/`k` | Navigate |
| `Tab` | Switch between panels |
| `Enter` | Apply selected mode |
| `Space` | Toggle option |
| `r` | Reset EnvyControl |
| `q` or `Esc` | Quit |

## Graphics Modes

| Mode | Description |
|------|-------------|
| **Integrated** | Uses Intel/AMD iGPU exclusively. Nvidia GPU is turned off for power saving. |
| **Hybrid** | Enables PRIME render offloading. GPU can be dynamically turned off when not in use. |
| **Nvidia** | Uses Nvidia dGPU exclusively. Higher performance, higher power consumption. |

## Options

### Hybrid Mode
- **RTD3 Power Management** - Enable PCI-Express Runtime D3 power management (Turing+)

### Nvidia Mode
- **Force Composition Pipeline** - Fix screen tearing
- **Coolbits** - Enable GPU overclocking options

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [EnvyControl](https://github.com/bayasdev/envycontrol) by Victor Bayas
- [Ratatui](https://github.com/ratatui-org/ratatui) - TUI library for Rust
