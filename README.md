# YamahaAmp

**YamahaAmp** est une bibliothèque Rust pour contrôler les amplificateurs Yamaha compatibles avec l'API **Yamaha Extended Control** (YXC). Elle propose une interface asynchrone et bloquante pour interagir avec les zones audio, régler le volume, changer les sources d'entrée, les programmes sonores, et plus encore.

---

## 🔧 Fonctionnalités

- 🎛 Découverte automatique des amplificateurs Yamaha sur le réseau local
- 🌐 Connexion directe via une IP
- 🎚 Contrôle du volume et de la sourdine
- 🎶 Changement de source d'entrée (`Input`)
- 🔊 Changement de programme sonore (`SoundProgram`)
- ✅ Support des appels **asynchrones** et **bloquants**
- 📦 Design ergonomique et orienté API

---

## 🚀 Utilisation

### Dépendance

Ajoute ceci à ton `Cargo.toml` :

```toml
yamaha_api = { path = "../yamaha_api" }
```

### Exemple (Async)

```rust
use yamaha_api::YamahaAmp;

#[tokio::main]
async fn main() {
    let amp = YamahaAmpAsync::connect(Ipv4Addr::new(192, 168, 1, 126)).await.expect("Failed to connect async");
    if let Some(amp) = amp {
        println!("Directly Connected async to {}", amp.info.model);

        if let Ok(e) = amp.get_zone_status(Zone::Main).await {
            match serde_json::from_value::<GetStatus>(e) {
                Ok(e) => println!("Main actual volume: {} / {}", e.volume, e.max_volume),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }

        if let Err(e) = amp.set_sound_program(SoundProgram::Straight).await {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Exemple (Blocking)

```rust
use yamaha_api::YamahaAmpBlocking;

fn main() {
        let amp = YamahaAmpBlocking::connect(Ipv4Addr::new(192, 168, 1, 126)).expect("Failed to connect sync");
        if let Some(amp) = amp {
            println!("Directly Connected sync to {}", amp.info.model);

            if let Ok(e) = amp.get_zone_status(Zone::Main) {
                println!("Main: {}", e);
            }

            if let Err(e) = amp.set_sound_program(SoundProgram::Straight) {
                eprintln!("Error: {:?}", e);
            }
        }
}
```

---

## 🔍 Découverte automatique

```rust
use yamaha_api::YamahaAmp;

#[tokio::main]
async fn main() {
    // Optional config
    let cfg = DiscoveryConfig {
        subnet: Ipv4Addr::new(192, 168, 1, 0),
        mask: 24,
        timeout: Duration::from_millis(200),
        max_concurrent: 50,
    };

    let amps = YamahaAmpAsync::discover(Some(cfg)).await.expect("Failed to discover amps async");
    for (index, amp) in amps.iter().enumerate() {
        println!("Async found {}. {} -> ({})", index + 1, amp.info.model, amp.ip, );
    }
}
```

Ou version bloquante :

```rust
use yamaha_api::YamahaAmpBlocking;

fn main() {
    // Optional config
    let cfg = DiscoveryConfig {
        subnet: Ipv4Addr::new(192, 168, 1, 0),
        mask: 24,
        timeout: Duration::from_millis(200),
        max_concurrent: 50,
    };

    let amps = YamahaAmpBlocking::discover(Some(cfg)).expect("Failed to discover amps sync");
    for (index, amp) in amps.iter().enumerate() {
        println!("Sync found {}. {} -> ({})", index + 1, amp.info.model, amp.ip);
    }
}
```

---

## 📚 Types Spécifiques

- `Input` : représente toutes les entrées disponibles (HDMI, USB, Bluetooth, etc.)
- `SoundProgram` : représente tous les programmes sonores supportés (Straight, Music, Game, etc.)
- `Zone` : support des différentes zones (`Main`, `Zone2`, etc.)
- `YamahaError` : gestion des erreurs d'API (code retour ≠ 0)

---
