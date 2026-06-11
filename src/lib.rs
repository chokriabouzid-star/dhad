//! # Dhad (ضاد) — Arabic Text Canonicalization Library
//!
//! Dhad implements a deterministic pipeline that converts Arabic Unicode
//! text into a canonical [`AtomStream`] of [`DhadAtom`]s and produces two
//! independent cryptographic hashes.
//!
//! - **[`hash::core_hash`]** — orthographic identity (base + marks + flags)
//! - **[`hash::phonetic_hash`]** — prosodic identity (CoreHash + prosody)
//!
//! ## Quick Start
//!
//! ```rust
//! use dhad::modes::process_mode_a;
//!
//! let result = process_mode_a("بِسْمِ".as_bytes()).unwrap();
//! println!("atoms:    {}", result.stream.len());
//! println!("core:     {}", hex::encode(result.core_hash));
//! println!("phonetic: {}", hex::encode(result.phonetic_hash));
//! ```
//!
//! ## Design Axioms
//!
//! | Axiom | Description |
//! |-------|-------------|
//! | A1 | Determinism — same input → identical output |
//! | A2 | No Silent Correction — invalid input → typed error |
//! | A3 | Hash Separation — CoreHash is independent of prosody |
//! | A5 | Glyph Independence — positional forms carry no information |
//! | A6 | Mark Order Independence — diacritic ordering is ignored |
//! | A7 | Digit Source Independence — ASCII/Arabic/Extended digits are equal |
//!
//! ## Important Limitations (v1.x)
//!
//! - **Input must be NFC-precomposed**. Decomposed forms such as
//!   `U+0627 U+0653` are not mapped in v1.x and will return
//!   `UnmappedCodepoint`.
//! - **Quranic annotation marks** such as `U+06D6`–`U+06ED` are out of scope
//!   for strict Mode A processing in v1.x.
//! - Some specification stages are **identity stages** in v1.x because
//!   normalization already occurs earlier in the pipeline.
//! - Mode B structural errors are currently reported as `MalformedUtf8`
//!   for API compatibility in v1.x.
//!
//! ## Specification Status
//!
//! Dhad v1.x is the reference implementation of the Dhad Implementation
//! Specification v1.0. Repository vectors are self-consistency regression
//! data; independent cross-implementation conformance is planned.
//!
//! See **Known Limitations** in `README.md` for full details.

pub(crate) mod base_map;
pub mod constants;
pub(crate) mod faps;
pub mod hash;
pub mod invariants;
pub mod mode_b;
pub mod model;
pub mod modes;
pub(crate) mod noise;
pub(crate) mod pipeline;
pub mod registry;

pub use model::{AtomStream, DhadAtom, DhadResult, ErrorKind};
pub use modes::{process_mode_a, process_mode_b};
