<h1 align="center"> Firmware Embarqué Rust - Embassy & Périphériqueso </h1>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-1.0-blue.svg?cacheSeconds=2592000" />
</p>

> Développement d’un firmware embarqué en Rust utilisant Embassy pour piloter plusieurs périphériques sur une carte Nucleo-64 L476RG

---

### 🏠 [Homepage](https://github.com/EtudiantEnsea)

# TP3&4_rust

Firmware embarqué modulaire en Rust basé sur Embassy permettant de piloter différents périphériques (LEDs, boutons, encodeur, moteur pas à pas, etc.).

---

## Description

`TP3&4_rust` consiste à développer un firmware embarqué structuré et réutilisable pour une carte d’extension ENSEA connectée à une Nucleo-64.

Le projet est découpé en plusieurs couches :

BSP (Board Support Package) : abstraction du hardware (mapping des pins)
Drivers : interfaces haut niveau pour chaque périphérique
Application : orchestration via des tâches asynchrones Embassy

Le firmware permet notamment de :

Lire un encodeur rotatif
Afficher une valeur sur un bargraph LED
Contrôler un moteur pas à pas
Lire un gamepad
Gérer un arrêt d’urgence

---

## Fonctionnalités

* BSP centralisant la configuration matérielle
* Driver Bargraph (affichage proportionnel)
* Driver Gamepad (lecture des boutons)
* Driver Encodeur (position + bouton via registres)
* Driver Stepper (direction, vitesse, microstepping)
* Communication inter-tâches via :
* AtomicU32
* Signal (Embassy)
* Gestion multi-tâches asynchrones avec Embassy
* Arrêt d’urgence via bouton

---

## Architecture

```
src/
├── .cargo/
│   ├── config.toml
├── blink/src/
|   ├── main.rs                # Application principale (tasks Embassy)
|   ├── drivers/
|   │   ├── mod.rs
|   │   ├── bsp_ensea.rs      # Mapping matériel (pins)
|   │   ├── bargraph.rs       # Driver LEDs
|   │   ├── gamepad.rs        # Driver boutons
|   │   ├── encoder.rs        # Driver encodeur (PAC)
|   │   └── stepper.rs        # Driver moteur pas à pas
|   ├── bin/
|   │   ├── bargraph_example.rs
|   │   ├── gamepad_example.rs
|   │   └── encoder_example.rs
├── memeory.x
```
---

## Utilisation

```sh
cargo run --bin bargraph_example
cargo run --bin gamepad_example
cargo run --bin encoder_example
```

---

## Prérequis

Rust installé
probe-rs installé
Carte STM32 Nucleo-64 L476RG
Carte d’extension ENSEA
---

## Auteur

**Loick GOMES GRANCHO & Abdoul Nouroudine SANA**
