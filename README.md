# rpi-pal – Raspberry Pi Peripheral Access Library

[![Build status](https://github.com/rpi-pal/rpi-pal/actions/workflows/ci.yml/badge.svg)](https://github.com/rpi-pal/rpi-pal/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/crates/v/rpi-pal)](https://crates.io/crates/rpi-pal)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.60.0-lightgray.svg)](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rpi-pal/latest)](https://deps.rs/crate/rpi-pal)


## Fork of RPPAL

This repository is a fork of [RPPAL](https://github.com/golemparts/rppal), which was archived on 2025-07-01. We will be
maintaining it here, to ensure that projects depending on it can continue.

## rpi-pal

rpi-pal provides access to the Raspberry Pi's GPIO, I2C, PWM, SPI and UART peripherals through a user-friendly
interface. In addition to peripheral access, rpi-pal also offers support for USB to serial adapters.

The library can be used in conjunction with a variety of platform-agnostic drivers through its `embedded-hal` trait
implementations. Both `embedded-hal` v0.2.7 and v1 are supported.

rpi-pal requires a recent release of Raspberry Pi OS. Similar Linux distributions may work, but are unsupported.
Both GNU and musl `libc` targets are supported. rpi-pal is compatible with the Raspberry Pi A, A+, B, B+, 2B, 3A+, 3B,
3B+, 4B, 5, CM, CM 3, CM 3+, CM 4, CM 5, CM 5 Lite, 400, 500, Zero, Zero W and Zero 2 W.

## Table of contents

- [Usage](#usage)
- [Examples](#examples)
- [Optional features](#optional-features)
- [Supported peripherals](#supported-peripherals)
  - [GPIO](#gpio)
  - [I2C](#i2c)
  - [PWM](#pwm)
  - [SPI](#spi)
  - [UART](#uart)
- [Cross compilation](#cross-compilation)
  - [Cargo](#cargo)
  - [Visual Studio Code](#visual-studio-code)
- [Caution](#caution)
- [Copyright and license](#copyright-and-license)

## Usage

Add a dependency for `rpi-pal` to your `Cargo.toml` using `cargo add rpi-pal`, or by adding the following line to your
dependencies section.

```toml
[dependencies]
rpi-pal = "0.22.1"
```

If your project requires `embedded-hal` trait implementations, specify either the `hal` or `hal-unproven` feature flag
in the dependency declaration.

```toml
[dependencies]
rpi-pal = { version = "0.22.1", features = ["hal"] }
```

Call `new()` on any of the peripherals to construct a new instance.

```rust
use rpi_pal::gpio::Gpio;
use rpi_pal::i2c::I2c;
use rpi_pal::pwm::{Channel, Pwm};
use rpi_pal::spi::{Bus, Mode, SlaveSelect, Spi};
use rpi_pal::uart::{Parity, Uart};

let gpio = Gpio::new()?;
let i2c = I2c::new()?;
let pwm = Pwm::new(Channel::Pwm0)?;
let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)?;
let uart = Uart::new(115_200, Parity::None, 8, 1)?;
```

Access to some peripherals may need to be enabled first through `sudo raspi-config` or by editing
`/boot/firmware/config.txt`. Refer to the relevant module's documentation for any required steps.

## Examples

This example demonstrates how to blink an LED connected to a GPIO pin. Remember to add a resistor of an appropriate
value in series, to prevent exceeding the maximum current rating of the GPIO pin and the LED.

```rust
use std::error::Error;
use std::thread;
use std::time::Duration;

use rpi_pal::gpio::Gpio;
use rpi_pal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}
```

Additional examples can be found in the `examples` directory.

## Optional features

By default, all optional features are disabled. You can enable a feature by specifying the relevant feature flag(s) in
the dependency declaration for `rpi-pal` in your `Cargo.toml`.

* `hal` - Enables `embedded-hal` trait implementations for all supported peripherals. This doesn't include `unproven` traits.
* `hal-unproven` - Enables `embedded-hal` trait implementations for all supported peripherals, including traits marked as `unproven`. Note that `embedded-hal`'s `unproven` traits don't follow semver rules. Patch releases may introduce breaking changes.

## Supported peripherals

### [GPIO](https://docs.rs/rppal/latest/rppal/gpio)

To ensure fast performance, rpi-pal controls the GPIO peripheral by directly accessing the registers through either
`/dev/gpiomem` or `/dev/mem`. GPIO interrupts are configured using the `gpiochip` character device.

#### Features

* Get/set pin mode and logic level
* Configure built-in pull-up/pull-down resistors
* Synchronous and asynchronous interrupt handlers
* Software-based PWM implementation
* Optional `embedded-hal` trait implementations

### [I2C](https://docs.rs/rppal/latest/rppal/i2c)

The Broadcom Serial Controller (BSC) peripheral controls a proprietary bus compliant with the I2C bus/interface. rpi-pal
communicates with the BSC using the `i2cdev` character device.

#### Features

* Single master, 7-bit slave addresses, transfer rates up to 400 kbit/s (Fast-mode)
* I2C basic read/write, block read/write, combined write+read
* SMBus protocols: Quick Command, Send/Receive Byte, Read/Write Byte/Word, Process Call, Block Write, PEC
* Optional `embedded-hal` trait implementations

### [PWM](https://docs.rs/rppal/latest/rppal/pwm)

rpi-pal controls the Raspberry Pi's PWM peripheral through the `pwm` sysfs interface.

#### Features

* Up to four hardware PWM channels depending on the Raspberry Pi model
* Configurable frequency, duty cycle and polarity
* Optional `embedded-hal` trait implementations

### [SPI](https://docs.rs/rppal/latest/rppal/spi)

rpi-pal controls the Raspberry Pi's main and auxiliary SPI peripherals through the `spidev` character device.

#### Features

* SPI master, mode 0-3, Slave Select active-low/active-high, 8 bits per word, configurable clock speed
* Half-duplex reads, writes, and multi-segment transfers
* Full-duplex transfers and multi-segment transfers
* Customizable options for each segment in a multi-segment transfer (clock speed, delay, SS change)
* Reverse bit order helper function
* Optional `embedded-hal` trait implementations

### [UART](https://docs.rs/rppal/latest/rppal/uart)

rpi-pal controls the Raspberry Pi's UART peripherals through the `ttyAMA0` (PL011) and `ttyS0` (mini UART) character
devices. USB to serial adapters are controlled using the `ttyUSBx` and `ttyACMx` character devices.

#### Features

* Support for UART peripherals (PL011, mini UART) and USB to serial adapters
* None/Even/Odd/Mark/Space parity, 5-8 data bits, 1-2 stop bits
* Transfer rates up to 4 Mbit/s (device-dependent)
* XON/XOFF software flow control
* RTS/CTS hardware flow control with automatic pin configuration
* Optional `embedded-hal` trait implementations

## Cross compilation

If you're not working directly on a Raspberry Pi, you'll have to cross-compile your code for the appropriate ARM
architecture. Check out [this guide](https://github.com/japaric/rust-cross) for more information, or try the [cross](https://github.com/japaric/cross) project for "zero setup" cross
compilation.

### Cargo

For manual cross-compilation without the use of `cross`, you will need to install the appropriate target. Most Raspberry
Pi models either need the `armv7-unknown-linux-gnueabihf` target for 32-bit Linux distributions, or
`aarch64-unknown-linux-gnu` for 64-bit. For some models, like the Raspberry Pi Zero, a different target triple is
required.

Install the relevant target using `rustup`.

```bash
rustup target install armv7-unknown-linux-gnueabihf
```

In the root directory of your project, create a `.cargo` subdirectory, and save the following snippet to
`.cargo/config.toml`.

```toml
[build]
target = "armv7-unknown-linux-gnueabihf"
```

### Visual Studio Code

The rust-analyzer extension for Visual Studio Code needs to be made aware of the target platform by setting the
`rust-analyzer.cargo.target` configuration option. In the root directory of your project, create a `.vscode`
subdirectory, and then save the following snippet to `.vscode/settings.json`.

```json
{
    "rust-analyzer.cargo.target": "armv7-unknown-linux-gnueabihf"
}
```

## Caution

Always be careful when working with the Raspberry Pi's peripherals, especially if you attach any external components to
the GPIO pins. Improper use can lead to permanent damage.

## Copyright and license

Copyright (c) 2017-2025 Rene van der Meer. Released under the [MIT license](LICENSE).
