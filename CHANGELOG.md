# Changelog

Wszystkie istotne zmiany w VantisOffice będą dokumentowane w tym pliku.

Format oparty na [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), a projekt przestrzega [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dodano
- Benchmarki wydajności dla Vantis Writer (10 benchmarków)
- Benchmarki wydajności dla Vantis Flow (14 benchmarków)
- Łącznie 143 benchmarki w 9 modułach
- Zjednoczona dokumentacja benchmarków (BENCHMARKS_COMPLETE.md)

### Zmieniono
- Usunięto zduplikowane dokumenty benchmarków
- Uproszczono strukturę dokumentacji

### Ulepszone
- Dokumentacja benchmarków jest teraz przejrzysta i kompletna
- Dostęp do wszystkich metryk wydajnościowych w jednym miejscu

---

## [0.3.0] - 2025-03-03

### Dodano
- **Performance Benchmarks**: 119 benchmarków w 7 modułach
  - Vantis Core-IO (11 benchmarków)
  - Vantis Vault (16 benchmarków)
  - Vantis Grid (15 benchmarków)
  - Vantis Canvas (19 benchmarków)
  - Vantis Link (21 benchmarków)
  - Vantis Chronos (18 benchmarków)
  - Vantis Ark (19 benchmarków)

- **User Guides**: Kompletna dokumentacja użytkownika
  - Vantis Writer Guide
  - Vantis Grid Guide
  - Vantis Canvas Guide
  - Vantis Lens Guide
  - Vantis Link Guide
  - Vantis Flow Guide
  - Vantis Chronos Guide
  - Vantis Ark Guide
  - Vantis Bridge Guide
  - User Guides Overview

### Ulepszone
- Dokumentacja API (rustdoc)
- Kompletny raport testów (116 testów, 98.3% pass rate)
- Wydajność operacji krytycznych zoptymalizowana

---

## [0.2.0] - 2025-03-02

### Dodano
- **Testy jednostkowe i integracyjne**: 116 testów w 10 modułach
  - Vantis Core-IO: 21 testów (100% pass)
  - Vantis Vault: 4 testy (100% pass)
  - Vantis Grid: 19 testów (17/19 pass)
  - Vantis Writer: 6 testów (100% pass)
  - Vantis Canvas: 16 testów (100% pass)
  - Vantis Lens: 14 testów (100% pass)
  - Vantis Link: 16 testów (100% pass)
  - Vantis Flow: 1 test (100% pass)
  - Vantis Ark: 9 testów (100% pass)
  - Vantis Bridge: 10 testów (100% pass)

- **Dokumentacja techniczna**
  - ARCHITECTURE.md - architektura systemu
  - API specs dla wszystkich modułów
  - Developer guides

- **CI/CD**
  - GitHub Actions workflow
  - Automatyczne testy przy każdym PR
  - Security audit z cargo-deny

---

## [0.1.0] - 2025-03-01

### Dodano
- **Pillar I - System Foundations** (Iron Layer)
  - Vantis Core-IO - obsługa plików z SHA-3 hashing
  - Vantis Vault - szyfrowanie TPM 2.0
  - WASM Sandbox - bezpieczne pluginy
  - Flux Vector Engine - rendering GPU z Vulkan

- **Pillar II - Productivity Applications** (Logic Layer)
  - Vantis Writer - procesor tekstu z Markdown
  - Vantis Grid - arkusz kalkulacyjny z AI
  - Vantis Canvas - prezentacje 3D
  - Vantis Lens - bezpieczny przeglądarka PDF

- **Pillar III - Ecosystem & Collaboration** (Sync Layer)
  - Vantis Link - P2P z CRDT
  - Vantis Flow - planowanie i diagramy
  - Vantis Chronos - kalendarz z PGP

- **Pillar IV - Critical Tools** (Continuity Layer)
  - Vantis Ark - backup rozproszony z Shamir
  - Vantis Bridge - konwerter formatów

- **Infrastruktura**
  - Workspace Cargo.toml
  - Podstawowe implementacje Rust
  - Przykładowe aplikacje
  - Dokumentacja architektury
  - Roadmap rozwoju

### Bezpieczeństwo
- TPM 2.0 encryption support
- WASM sandboxing dla pluginów
- End-to-end encryption dla współpracy
- Shamir Secret Sharing dla backupów

---

[Unreleased]: https://github.com/vantisCorp/VantisOffice/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/vantisCorp/VantisOffice/releases/tag/v0.3.0
[0.2.0]: https://github.com/vantisCorp/VantisOffice/releases/tag/v0.2.0
[0.1.0]: https://github.com/vantisCorp/VantisOffice/releases/tag/v0.1.0