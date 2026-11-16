#![doc = include_str!("../README.md")]
#![no_std]

use aligned::{A4, Aligned};

/// CYW43439 Wi-Fi firmware, aligned to 4 bytes.
///
/// Ready to pass directly to `cyw43::new(..., &FW, NVRAM)`.
///
/// # License
/// See [README.md](../README.md#licenses) and
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub static FW: Aligned<A4, [u8; 231077]> =
    Aligned(*cyw43_firmware::CYW43_43439A0);

/// CYW43439 CLM (Country Locale Matrix) blob.
///
/// Pass to `control.init(CLM).await`.
///
/// # License
/// See [README.md](../README.md#licenses) and
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub static CLM: &[u8] =
    cyw43_firmware::CYW43_43439A0_CLM;

/// CYW43439 Bluetooth firmware.
///
/// # License
/// See [README.md](../README.md#licenses) and
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "bluetooth")]
pub static BTFW: &[u8] =
    cyw43_firmware::CYW43_43439A0_BTFW;

/// NVRAM configuration for RP2040 / RP2350.
///
/// Ready to pass directly to `cyw43::new(..., &FW, &NVRAM)`.
///
/// # License
/// See [README.md](../README.md#licenses).


#[cfg(feature = "wifi")]
pub static NVRAM: Aligned<A4, [u8; 742]> =
    Aligned(*include_bytes!("../firmware/nvram_rp2040.bin"));