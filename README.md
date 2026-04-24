# gfetch-rs

A lightweight, fast system information fetch tool written in Rust — inspired by neofetch, but minimal and blazing fast.

```
    (\(\       meizfl@pc.meizfl.pp.ua
   j". ..      -----------
   (  . .)     os: openSUSE Tumbleweed 7.0.0-6.g11c9579-default (x86_64)
   |   ° ¡     shell: /usr/bin/zsh
   ¿     ;     uptime: 3d 4h 36m
   c?".UJ      ram: 11120/32010 MiB
               cpu: AMD Ryzen 7 3700X 8-Core Processor
               resolution: 1920x1080
               fs: ext4
               storage: 326 / 466 GB
```

## Features

- Displays system info alongside a small ASCII bunny
- Shows OS name, kernel version, and architecture
- Reports CPU model, RAM usage, uptime, shell, screen resolution, filesystem type, and disk usage
- Automatically detects the most relevant disk (based on the executable or working directory path)
- Cross-platform support (Linux, macOS, Windows)

## Displayed Information

| Field | Description |
|---|---|
| `user@host` | Current username and hostname |
| `os` | OS name, kernel version, and CPU architecture |
| `shell` | Current shell from `$SHELL` (or `COMSPEC` on Windows) |
| `uptime` | System uptime formatted as `Xd Xh Xm` |
| `ram` | Used / Total RAM in MiB |
| `cpu` | CPU brand name |
| `resolution` | Screen resolution(s); multiple monitors separated by ` \| ` |
| `fs` | Filesystem type of the current disk |
| `storage` | Used / Total disk space in GB |

## Requirements

- [Rust](https://rustup.rs/) 1.70 or later

## Dependencies

The following crates are used:

```toml
[dependencies]
sysinfo = "*"
whoami = "*"
xcap = "*"
```

## Building

```bash
git clone https://github.com/meizfl/gfetch-rs
cd gfetch-rs
cargo build --release
```

The compiled binary will be at `target/release/gfetch`.

## Running

```bash
./target/release/gfetch
```

Or install it to your PATH:

```bash
cargo install --path .
```

## Project Structure

```
Cargo.toml
src/
└── main.rs      # All logic: system info collection, formatting, and output
```

## License

GNU/GPL3
