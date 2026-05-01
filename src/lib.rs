#![doc = include_str!("../README.md")]
#![no_std]

use aligned::{A4, Aligned};

/// Firmware Wi-Fi du CYW43439, aligné sur 4 octets.
///
/// Prêt à passer directement à `cyw43::new(..., &FW, NVRAM)`.
///
/// # Licence
/// Voir [README.md](../README.md#licences) et
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub static FW: Aligned<A4, [u8; 231077]> =
    Aligned(*cyw43_firmware::CYW43_43439A0);

/// Blob CLM (Country Locale Matrix) du CYW43439.
///
/// À passer à `control.init(CLM).await`.
///
/// # Licence
/// Voir [README.md](../README.md#licences) et
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub static CLM: &[u8] =
    cyw43_firmware::CYW43_43439A0_CLM;

/// Firmware Bluetooth du CYW43439.
///
/// # Licence
/// Voir [README.md](../README.md#licences) et
/// <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "bluetooth")]
pub static BTFW: &[u8] =
    cyw43_firmware::CYW43_43439A0_BTFW;

/// Configuration NVRAM pour RP2040 / RP2350.
///
/// Prêt à passer directement à `cyw43::new(..., &FW, NVRAM)`.
///
/// # Licence
/// Voir [README.md](../README.md#licences).


#[cfg(feature = "wifi")]
pub static NVRAM: Aligned<A4, [u8; 742]> =
    Aligned(*include_bytes!("../firmware/nvram_rp2040.bin"));