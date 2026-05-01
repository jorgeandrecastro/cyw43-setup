# cyw43-setup

[![Crates.io](https://img.shields.io/crates/v/cyw43-setup.svg)](https://crates.io/crates/cyw43-setup)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org)
[![docs.rs](https://docs.rs/cyw43-setup/badge.svg)](https://docs.rs/cyw43-setup)

Wrapper de commodité au-dessus de [`cyw43-firmware`](https://crates.io/crates/cyw43-firmware).

`cyw43-firmware` inclut les blobs binaires du CYW43439 mais n'expose aucune API Rust alignée.  
Cette crate expose `FW`, `CLM`, `NVRAM` et `BTFW` comme constantes statiques directement utilisables avec `cyw43`.

>Projet communautaire non-officiel. Non affilié à Raspberry Pi Ltd, Broadcom ou Infineon, pensé uniquement pour améliorer l’expérience développeur et simplifier l’intégration. 

---

## Le problème résolu

**Avant  dans chaque projet :**
```rust
// Cloner le repo firmware manuellement, gérer les chemins relatifs...
let fw    = aligned_bytes!("../cyw43-firmware/43439A0.bin");
let clm   = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");
```

**Après avec cette crate qui wrappe la crate cyw43-firmware :**
```rust
use cyw43_setup::{FW, CLM, NVRAM};
// C'est tout.
```

Aucun repo à cloner, aucun chemin relatif à gérer, aucun `aligned_bytes!`, aucun binaire à copier.
Ajoute la dépendance dans `Cargo.toml` et c’est prêt.
---

## 📦 Installation

```toml
[dependencies]
cyw43-setup = "0.1"
cyw43 = "0.3"
```

Pour le support Bluetooth :
```toml
[dependencies]
cyw43-setup = { version = "0.1", features = ["bluetooth"] }
```

---

## 🚀 Utilisation

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use cyw43::JoinOptions;
use cyw43_setup::{FW, CLM, NVRAM};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // ... init PIO, SPI, etc. ...

    let (net_device, mut control, runner) =
        cyw43::new(STATE.init(cyw43::State::new()), pwr, spi, &FW, &NVRAM).await;// & est requis car cyw43::new attend une référence alignée

    spawner.spawn(cyw43_runner_task(runner)).unwrap();

    control.init(CLM).await;
    control.join("mon-wifi", JoinOptions::new(b"mon-mdp")).await.ok();
}
```

---

## Constantes exposées
| Constante | Feature | Type | Usage |
|---|---|---|---|
| `FW` | `wifi` (défaut) | `Aligned<A4, [u8; 231077]>` | `cyw43::new(..., &FW, NVRAM)` |
| `CLM` | `wifi` (défaut) | `&[u8; 984]` | `control.init(CLM).await` |
| `NVRAM` | `wifi` (défaut) | `Aligned<A4, [u8; 742]>` | `cyw43::new(..., &FW, &NVRAM)` |
| `BTFW` | `bluetooth` | `&[u8; 6164]` | firmware Bluetooth |

---

## ⚠️ Licences firmware

Les blobs `43439A0.bin`, `43439A0_clm.bin` et `nvram_rp2040.bin` sont la propriété de
**George Robotics Pty Ltd**. Sources :
- [georgerobotics/cyw43-driver](https://github.com/georgerobotics/cyw43-driver/tree/main/firmware)
- [embassy-rs/cyw43-firmware](https://github.com/embassy-rs/cyw43-firmware)

Ils sont disponibles sous deux licences au choix :

| Licence | Usage |
|---|---|
| `LICENSE.RP` | Utilisation avec matériel RP2040/RP2350 uniquement |
| `LICENSE.NC` | Utilisation non-commerciale uniquement |

➡️ Pour un usage Pico W / Pico 2W personnel : tu es couvert par les deux.

---

## 🛡 Licences

| Composant | Licence |
|---|---|
| Code Rust de cette crate | [Unlicense](LICENSE) (domaine public) |
| Wrapper `cyw43-firmware` | [Unlicense](https://unlicense.org) |
| Blobs firmware | George Robotics Pty Ltd (`LICENSE.RP` / `LICENSE.NC`) |

---

## À propos

Développé par **Jorge Andre Castro**.  
Tout crédit revient à **KizzyCode / George Robotics / embassy-rs** pour les blobs firmware et le wrapper original.