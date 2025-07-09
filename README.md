# rust-corne-firmware

A simple corne keyboard firmware in rust

Note: `esp8266-hal` is deprecated and missing important features like WiFi.
But I was still interested if this would work at all.

# Requirements

- rustup (>= 1.27.1)
- rust (>= 1.82.0)
- cargo (>= 1.82.0)
- cargo-espflash (exactly 2.1.0)
  Can be installed via `cargo binstall cargo-espflash@2.1.0`
- Rust xtensa-lx106 toolchain
  Can be installed via `espup install --toolchain-version 1.80.0.0`

# Build

Make sure to source the esp toolchain:

```bash
source ~/export-esp.sh
```

```bash
cargo build
# or:
cargo build --release
```

# Flash to ESP-01S

```bash
cargo espflash flash --baud=115200 --flash-mode=qio --flash-freq=40mhz --flash-size=1mb --port=/dev/ttyUSB1 --release --monitor
```
