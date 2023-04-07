## ~~NimBLE Rust wrapper for ESP32~~
# Bluetooth Low Energy (BLE) for ESP32 using ESP-IDF-SYS (std lib required)


OLD badges from fork:
[![crates.io](https://img.shields.io/crates/v/esp32-nimble)](https://crates.io/crates/esp32-nimble)
[![build](https://github.com/taks/esp32-nimble/actions/workflows/ci.yml/badge.svg)](https://github.com/taks/esp32-nimble/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/esp32-nimble)](https://github.com/taks/esp32-nimble/blob/develop/LICENSE)
[![Documentation](https://img.shields.io/badge/docs-esp32--nimble-brightgreen)](https://taks.github.io/esp32-nimble/esp32_nimble/index.html)

This is a Rust wrapper for the NimBLE Bluetooth stack for ESP32, forked from https://github.com/taks/esp32-nimble.

It is tested exclusively on an Adafruit ESP32 Feather board, but theoretically should work on any ESP32 board.

## Usage
You must be in a project using `esp-idf-sys` with std lib, see this template to get started:
```sh
cargo generate https://github.com/esp-rs/esp-idf-template cargo
```

Add below settings to your project's `sdkconfig.defaults`.
```
CONFIG_BT_ENABLED=y
CONFIG_BT_BLE_ENABLED=y
CONFIG_BT_BLUEDROID_ENABLED=n
CONFIG_BT_NIMBLE_ENABLED=y
```

See [examples](./examples) for usage.

<!-- TODO: Add more documentation in README.md -->

## Features
- [x] GATT server
  - [x] Advertisement
  - [x] Services
  - [x] Characteristics
    - [x] Read
    - [x] Write
    - [x] Notify
    - [x] Indicate
  - [x] Descriptors
    - [x] Read
    - [x] Write
  - [x] Encryption
- [x] GATT client
  - [x] Scan
  - [x] Services
  - [x] Characteristics
    - [x] Read
    - [x] Write
    - [x] Notify
    - [x] Indicate
  - [x] Descriptors
    - [x] Read
    - [x] Write
  - [x] Encryption
