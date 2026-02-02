# 💎 Stonix: Telebasix Fast Storage Engine

[![Security](https://img.shields.io/badge/Security-AES--256-blue)](#)

## 🇪🇸 Castellano
Motor NoSQL ultra-ligero para la infraestructura **TELARIX**. Eficiencia extrema.

## 🇺🇸 English
Ultra-lightweight NoSQL engine for **TELARIX** infrastructure. Extreme efficiency.

## 🇩🇪 Deutsch
Ultraleichte NoSQL-Engine für die **TELARIX**-Infrastruktur. Extreme Effizienz.

## 🇷🇺 Русский
Ультралегкий движок NoSQL для инфраструктуры **TELARIX**. Максимальная эффективность.

## 🇨🇳 中文
专为 **TELARIX** 基础设施设计的超轻量级 NoSQL 引擎。极致效率。
## 🛠 Compilación / Compilation / Kompilierung / Компиляция / 编译

### Standard Build
```bash
cargo build --release
```

### ARM Cross-compilation
```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

## 🚀 Quick Start (Uso Rápido)

Añade Stonix a tu proyecto:
```bash
cargo add stonix
```

### Ejemplo de uso: Almacenamiento Cifrado
Stonix protege tus datos automáticamente usando AES-256-GCM.

```rust
// Ejemplo básico de uso en TELARIX
fn main() -> anyhow::Result<()> {
    // El motor Stonix se encarga del resto
    println!("Motor Stonix v0.1.1 operativo.");
    Ok(())
}
```
