# Third-Party Licenses

The Apache License 2.0 in [LICENSE](LICENSE) covers only the source code authored by
this project (`src/`, `src-tauri/src/`, `scripts/`, `tools/`, `.github/`).
Everything listed on this page belongs to its own rights holder and is governed by
its own terms. This project asserts no ownership over any of it.

See [NOTICE](NOTICE) for the attribution summary.

---

## 1. Local language model

| Item | Value |
| --- | --- |
| Model | `gemma-2-2b-it` (GGUF, Q4_K_M) |
| Rights holder | Google LLC |
| Terms | [Gemma Terms of Use](https://ai.google.dev/gemma/terms) |
| Use policy | [Gemma Prohibited Use Policy](https://ai.google.dev/gemma/prohibited_use_policy) |
| GGUF conversion | [`bartowski/gemma-2-2b-it-GGUF`](https://huggingface.co/bartowski/gemma-2-2b-it-GGUF) |
| Included in this repository | **No** — `ai/` is git-ignored; no weights are committed or redistributed |
| How it reaches the user | The app downloads it at runtime on the end user's own machine ([`download.rs`](src-tauri/src/infrastructure/llm/download.rs)) |

Because this project never distributes the weights, the Gemma Terms of Use form a
direct agreement between Google and the person who downloads the model. Any
obligation arising from using Gemma — including the use restrictions in Section 3.2
of those Terms — rests with that user.

## 2. EverSoul game resources

| Item | Location in repository |
| --- | --- |
| Spirit illustrations (base / costume / gacha / raid / srg) | `public/eversoul-assets/spirits/` |
| Talk backgrounds | `public/eversoul-assets/backgrounds/` |
| UI elements and race badges | `public/eversoul-assets/ui/` |
| Spirit profile source data | `data/personas/`, `src-tauri/resources/personas.bin` |
| Spirit voice lines | `src-tauri/resources/voices.bin` |

Copyright in these works belongs to the original rights holders of EverSoul.
This project uses them as a non-commercial fan project and claims no rights to
them. They are **not** covered by the Apache License 2.0.

## 3. Runtime dependencies

Rust crates and npm packages are pulled from crates.io and the npm registry under
their own licenses. Their terms are recorded in [`src-tauri/Cargo.lock`](src-tauri/Cargo.lock)
and [`package-lock.json`](package-lock.json), and each package carries its license
in its own distribution. Notable ones that ship inside the built executable:

| Component | License |
| --- | --- |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) (via `llama-cpp-2` / `llama-cpp-sys-2`) | MIT |
| [Tauri](https://tauri.app/) v2 | Apache-2.0 OR MIT |
| [candle](https://github.com/huggingface/candle) (`candle-core`, `candle-nn`) | Apache-2.0 OR MIT |
| [tokenizers](https://github.com/huggingface/tokenizers) | Apache-2.0 |
| [SQLite](https://sqlite.org/) (bundled via `rusqlite`) | Public Domain |
| [React](https://react.dev/) | MIT |
| [lucide-react](https://lucide.dev/) | ISC |
