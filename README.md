# garden-gateway

[![CI](https://github.com/JeffLabonte/garden-gateway/actions/workflows/ci.yml/badge.svg)](https://github.com/JeffLabonte/garden-gateway/actions/workflows/ci.yml)

Note that this is still a WORK IN PROGRESS project. I am learning Rust while doing this project.

## Getting Started

This project currently runs only on Raspberry PI.

It was tested on Debian Buster (Raspberry Pi OS)

## Requirements

* ![nix](https://github.com/NixOS/nixpkgs)

* `cargo`
* `rust`
* `sqlite3-dev`
  * `sudo apt install libsqlite3-dev`

### Create .env file

1. Run `make copy_template`
2. Edit `.env` using your favorite editor
3. Update `DATABASE_URL`

### Integrated Sensors/Devices

1. [IoT Relay](https://www.amazon.ca/gp/product/B00WV7GMA2/ref=ppx_yo_dt_b_asin_title_o05_s00?ie=UTF8&psc=1)

### Features

| Status | Feature |
|:------:|:-------:|
| :white_check_mark: | List Configurations |
| :white_check_mark: | Integrate IoT Relay |
| :white_check_mark: | Import Schedules from JSON file |
| :x: | Integrate custom board to switch Pump** |
| :x: | Import Configurations from JSON file |

**Board using [BC517 transistor](https://www.digikey.ca/en/products/detail/onsemi/BC517-D74Z/976355) for the [submersible water pump.](https://www.digikey.ca/en/products/detail/adafruit-industries-llc/4547/11627730)
