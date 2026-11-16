# cyw43-setup

[![Crates.io](https://img.shields.io/crates/v/cyw43-setup.svg)](https://crates.io/crates/cyw43-setup)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org)
[![docs.rs](https://docs.rs/cyw43-setup/badge.svg)](https://docs.rs/cyw43-setup)

Convenience wrapper for [`cyw43-firmware`](https://crates.io/crates/cyw43-firmware).

`cyw43-firmware` includes the CYW43439 binary blobs but does not expose an aligned Rust API.
This crate exposes `FW`, `CLM`, `NVRAM`, and `BTFW` as static constants that can be used directly with `cyw43`.

>Unofficial community project. Not affiliated with Raspberry Pi Ltd, Broadcom, or Infineon. Intended only to improve the developer experience and simplify integration.

---

## The problem this solves

**Before, in every project:**
```rust
// Manually clone the firmware repository and manage relative paths...
let fw    = aligned_bytes!("../cyw43-firmware/43439A0.bin");
let clm   = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");
```

**After, with this crate wrapping `cyw43-firmware`:**
```rust
use cyw43_setup::{FW, CLM, NVRAM};
// That's it.
```

No repository to clone, no relative paths to manage, no `aligned_bytes!`, and no binaries to copy.
Add the dependency to `Cargo.toml` and you're ready to go.
---

## 📦 Installation

```toml
[dependencies]
cyw43-setup = "0.2"
cyw43 = "0.3"
```

For Bluetooth support:
```toml
[dependencies]
cyw43-setup = { version = "0.2", features = ["bluetooth"] }
```

---

## 🚀 Usage

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use cyw43::JoinOptions;
use cyw43_setup::{FW, CLM, NVRAM};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // ... initialize PIO, SPI, etc. ...

    let (net_device, mut control, runner) =
        cyw43::new(STATE.init(cyw43::State::new()), pwr, spi, &FW, &NVRAM).await; // `&` is required because `cyw43::new` expects aligned references

    spawner.spawn(cyw43_runner_task(runner)).unwrap();

    control.init(CLM).await;
    control.join("my-wifi", JoinOptions::new(b"my-password")).await.ok();
}
```

---

## Exposed constants
| Constant | Feature | Type | Usage |
|---|---|---|---|
| `FW` | `wifi` (default) | `Aligned<A4, [u8; 231077]>` | `cyw43::new(..., &FW, &NVRAM)` |
| `CLM` | `wifi` (default) | `&[u8; 984]` | `control.init(CLM).await` |
| `NVRAM` | `wifi` (default) | `Aligned<A4, [u8; 742]>` | `cyw43::new(..., &FW, &NVRAM)` |
| `BTFW` | `bluetooth` | `&[u8; 6164]` | Bluetooth firmware |

---

## ⚠️ Firmware licenses

The `43439A0.bin`, `43439A0_clm.bin`, and `nvram_rp2040.bin` blobs are the property of
**George Robotics Pty Ltd**. Sources:
- [georgerobotics/cyw43-driver](https://github.com/georgerobotics/cyw43-driver/tree/main/firmware)
- [embassy-rs/cyw43-firmware](https://github.com/embassy-rs/cyw43-firmware)

They are available under either of these two licenses:

| License | Usage |
|---|---|
| `LICENSE.RP` | Use with RP2040/RP2350 hardware only |
| `LICENSE.NC` | Non-commercial use only |

➡️ For personal use with a Pico W / Pico 2 W, both licenses cover you.

---

## 🛡 Licenses

| Component | License |
|---|---|
| This crate's Rust code | [Unlicense](LICENSE) (public domain) |
| `cyw43-firmware` wrapper | [Unlicense](https://unlicense.org) |
| Firmware blobs | George Robotics Pty Ltd (`LICENSE.RP` / `LICENSE.NC`) |

---

## About

Developed by **Jorge Andre Castro**.
All credit for the firmware blobs and original wrapper goes to **KizzyCode / George Robotics / embassy-rs**.