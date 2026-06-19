# NFramework Core CLI (Rust)

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

CLI building blocks for the NFramework Rust toolchain. This workspace defines technology-agnostic CLI abstractions and provides adapters that implement them on top of popular Rust crates, following NFramework's zero-dependency-core, replaceable-adapter model.

The `nfw` CLI is built on these crates.

---

## Crates

| Crate | Role |
| --- | --- |
| `n-framework-core-cli-abstractions` | Core CLI contracts — command specs, options, runtime traits. No third-party CLI dependencies. |
| `n-framework-core-cli-clap` | Adapter implementing the runtime on top of [clap](https://crates.io/crates/clap). |
| `n-framework-core-cli-inquire` | Adapter for interactive prompts via [inquire](https://crates.io/crates/inquire). |
| `n-framework-core-cli-cliclack` | Adapter for interactive prompts via [cliclack](https://crates.io/crates/cliclack). |

Abstractions never depend on adapters; swapping a prompt or parsing backend does not touch consumer code.

---

## Build

```bash
make build
```

## Test

```bash
make test
```

## Format & Lint

```bash
make format
make lint
```

## Setup

```bash
make setup
```

---

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
