# Changelog

Wszystkie istotne zmiany w VantisOffice będą dokumentowane w tym pliku.

Format oparty na [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), a projekt przestrzega [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2025-03-04

### Dodano
- **Testy dla vantis-lens** (21 testów)
  - PDF document creation, page management
  - Sterilization (metadata, JavaScript, external links removal)
  - Annotations (creation, positioning, types)
  - Complete sterilization workflow integration tests
- **Testy dla vantis-link** (28 testów)
  - User management (creation, roles, online status)
  - Session management (users, max limits, metadata)
  - Document operations (insert, delete, replace changes)
  - Collaboration workflow integration tests
- **PartialEq derives** dla AnnotationType, UserRole, ChangeType

### Bezpieczeństwo
- **Usunięto rust-crypto** z vantis-grid (2 krytyczne podatności)
  - Zastąpiono nowoczesnymi bibliotekami RustCrypto: sha2, aes, rand
- **Zaktualizowano wasmtime** z 15.0 do 24.0 (4 krytyczne podatności)
  - Wszystkie podatności WASM sandbox naprawione
- **Audit bezpieczeństwa**: 0 podatności (z 9 wcześniej)

### Zmieniono
- **Łącznie 187 testów** w 12 modułach (z 138)
- **100% pass rate** dla wszystkich testów
- **12 modułów z testami** (z 10)

### Statystyki
- Testy: 187 (100% passing)
- Moduły z testami: 12/14 (86%)
- Bezpieczeństwo: 0 podatności

---

## [0.4.0] - 2025-03-04

### Dodano
- **Benchmarki wydajnościowe** dla 4 modułów (64 nowe benchmarki)
  - Vantis Lens (16 benchmarków) - PDF rendering, sterilization, signing
  - Vantis Bridge (18 benchmarków) - format conversion, sanitization
  - WASM Sandbox (16 benchmarków) - plugin execution, capabilities
  - Flux Vector Engine (15 benchmarków) - GPU rendering, vector graphics
- **Testy jednostkowe dla Vantis Chronos** (30 testów)
  - Event management, calendar queries, recurrence
  - Encryption, key management, conflict detection
  - Scheduling optimization, availability checking

### Naprawiono
- **138 testów przechodzących** w 10 modułach (100% pass rate)
  - Naprawiono vantis-chronos (269 błędów → 0)
  - Naprawiono importy vantis-vault, vantis-writer, wasm-sandbox
  - Wszystkie testy działają poprawnie
- **Formatowanie kodu** - 121 plików sformatowanych rustfmt
- **Warningi kompilatora** - naprawiono wszystkie warningi w testach

### Zmieniono
- Łącznie 207 benchmarków w 13 modułach (z 143 w 9 modułach)
- Zaktualizowano BENCHMARKS_COMPLETE.md do wersji 2.0
- Ulepszona dokumentacja z nowymi wykresami i tabelami

### Ulepszone
- Pokrycie benchmarków na poziomie 93% (13/14 modułów)
- Zwiększona czytelność dokumentacji technicznej
- Jakość kodu - wszystkie testy przechodzą
- Pokrycie testami: 138 testów w 10 modułach

---

## [Unreleased]

### Dodano
- **Benchmarki wydajnościowe** dla 4 modułów (64 nowe benchmarki)
  - Vantis Lens (16 benchmarków) - PDF rendering, sterilization, signing
  - Vantis Bridge (18 benchmarków) - format conversion, sanitization
  - WASM Sandbox (16 benchmarków) - plugin execution, capabilities
  - Flux Vector Engine (15 benchmarków) - GPU rendering, vector graphics
- **Testy jednostkowe dla Vantis Chronos** (30 testów)
  - Event management, calendar queries, recurrence
  - Encryption, key management, conflict detection
  - Scheduling optimization, availability checking
- **Zaawansowany README.md** - najbardziej zaawansowany na świecie
  - 8 języków (PL, EN, DE, ZH, RU, KO, ES, FR)
  - 26 funkcji (A-Z) - animacje, diagramy, interaktywne elementy
  - Black & Red color scheme, GitHub Pro features

### Zmieniono
- Łącznie 207 benchmarków w 13 modułach (z 143 w 9 modułach)
- Zaktualizowano BENCHMARKS_COMPLETE.md do wersji 2.0
- Ulepszona dokumentacja z nowymi wykresami i tabelami

### Ulepszone
- Pokrycie benchmarków na poziomie 93% (13/14 modułów)
- Zwiększona czytelność dokumentacji technicznej
- Profesjonalny światowej klasy projekt README

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